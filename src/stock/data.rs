use crate::types::request::stock::subscribe::{SubscribeRequest, TrType};
use crate::types::response::stock::subscribe::SubscribeResponse;
use crate::types::stream::stock::{MyExec, StreamParser};
use crate::types::{Account, CustomerType, Environment, TrId};
use crate::{auth, Error};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message,
    MaybeTlsStream, WebSocketStream,
};

type WsStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
type WsSplitStream = futures_util::stream::SplitStream<WsStream>;
type _WsSplitSink = futures_util::stream::SplitSink<WsStream, Message>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct KoreaStockData {
    exec_url: String,
    ordb_url: String,
    my_exec_url: String,
    environment: Environment,
    auth: auth::Auth,
    account: Account,
    hts_id: String,
    handles: HashMap<TrId, tokio::task::JoinHandle<()>>,
}

impl KoreaStockData {
    /// 국내 주식 실시간 시세에 관한 API
    /// [실시간시세(국내주식)](https://apiportal.koreainvestment.com/apiservice-apiservice?/tryitout/H0STCNT0)
    pub fn new(
        environment: Environment,
        auth: auth::Auth,
        account: Account,
        hts_id: &str,
    ) -> Result<Self, Error> {
        let endpoint_url = match environment {
            Environment::Real => "ws://ops.koreainvestment.com:21000",
            Environment::Virtual => "ws://ops.koreainvestment.com:31000",
        };
        let exec_url = format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(TrId::RealtimeExec),
        );
        let ordb_url = format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(TrId::RealtimeOrdb),
        );
        let my_exec_url = format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(match environment {
                Environment::Real => TrId::RealRealtimeMyExec,
                Environment::Virtual => TrId::VirtualRealtimeMyExec,
            }),
        );

        Ok(Self {
            exec_url,
            ordb_url,
            my_exec_url,
            environment,
            auth,
            account,
            hts_id: hts_id.to_string(),
            handles: HashMap::new(),
        })
    }

    /// 종목 시세 구독
    pub async fn subscribe_market<T: StreamParser<R> + Send, R: Clone + Send>(
        &mut self,
        tr_key: &str,
        tr_id: TrId,
    ) -> Result<
        (
            Option<tokio::sync::mpsc::UnboundedReceiver<T>>,
            SubscribeResponse,
        ),
        Error,
    > {
        let app_key = self.auth.get_appkey();
        let app_secret = self.auth.get_appsecret();
        let personalseckey = self.auth.get_approval_key().unwrap();
        let msg_str = SubscribeRequest::new(
            app_key,
            app_secret,
            personalseckey,
            CustomerType::Personal,
            tr_key.to_string(),
            tr_id.clone(),
            TrType::Register,
        )
        .get_json_string();

        let url = match tr_id {
            TrId::RealtimeExec => self.exec_url.clone(),
            TrId::RealtimeOrdb => self.ordb_url.clone(),
            _ => return Err(Error::WrongTrId(tr_id, "RealtimeExec or RealtimeOrdb")),
        };

        let (ws_stream, _) = connect_async(&url).await?;
        let (mut write, mut read) = ws_stream.split();

        write.send(Message::Text(msg_str)).await?;

        let mut result = SubscribeResponse::new(false, "".to_string(), None, None);
        send_subscribe_msg(&mut read, &mut result).await?;

        let handle_ref = self.handles.get(&tr_id);
        if handle_ref.is_none() || handle_ref.unwrap().is_finished() {
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
            let handle = tokio::spawn(async move {
                loop {
                    match read.next().await {
                        Some(Ok(Message::Text(s))) => {
                            debug!("Get message from stream={:?}", s);
                            let data = T::parse(s.clone()).expect("Failed to parse message");
                            if *data.header().tr_id() == TrId::PingPong {
                                let _ = write.send(Message::Text(s)).await;
                            } else {
                                let _ = tx.send(data);
                            }
                        }
                        Some(Ok(_)) => {
                            error!("Get wrong data from stream");
                            break;
                        }
                        Some(Err(e)) => {
                            error!("Failed to get message from stream: {:?}", e);
                            break;
                        }
                        None => break,
                    }
                }
            });
            self.handles.insert(tr_id, handle);
            return Ok((Some(rx), result));
        }
        Ok((None, result))
    }

    /// 체결통보 구독
    pub async fn subscribe_my_exec(
        &mut self,
    ) -> Result<
        (
            tokio::sync::mpsc::UnboundedReceiver<MyExec>,
            SubscribeResponse,
        ),
        Error,
    > {
        let app_key = self.auth.get_appkey();
        let app_secret = self.auth.get_appsecret();
        let personalseckey = self.auth.get_approval_key().expect("No approval key");
        let tr_id = match self.environment {
            Environment::Real => TrId::RealRealtimeMyExec,
            Environment::Virtual => TrId::VirtualRealtimeMyExec,
        };
        let msg_str = SubscribeRequest::new(
            app_key,
            app_secret,
            personalseckey,
            CustomerType::Personal,
            self.hts_id.clone(),
            tr_id.clone(),
            TrType::Register,
        )
        .get_json_string();

        let (ws_stream, _) = connect_async(&self.my_exec_url).await?;
        let (mut write, mut read) = ws_stream.split();

        write.send(Message::Text(msg_str)).await?;

        let mut result = SubscribeResponse::new(false, "".to_string(), None, None);
        send_subscribe_msg(&mut read, &mut result).await?;

        if let Some(handle) = self.handles.remove(&tr_id) {
            handle.abort();
        }

        let (iv, key) = (
            result.iv().clone().expect("no iv"),
            result.key().clone().expect("no key"),
        );
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let handle = tokio::spawn(async move {
            loop {
                match read.next().await {
                    Some(Ok(Message::Text(s))) => {
                        let data = MyExec::parse(s.clone(), iv.clone(), key.clone())
                            .expect("Failed to parse message");
                        if data.header().tr_id() == &TrId::PingPong {
                            let _ = write.send(Message::Text(s)).await;
                        } else {
                            let _ = tx.send(data);
                        }
                    }
                    Some(Ok(_)) => {
                        error!("Get wrong data from stream");
                        break;
                    }
                    Some(Err(e)) => {
                        error!("Failed to get message from stream: {:?}", e);
                        break;
                    }
                    None => break,
                }
            }
        });
        self.handles.insert(tr_id, handle);
        Ok((rx, result))
    }

    /// 종목 시세 구독 해체
    pub async fn unsubscribe_market(
        &mut self,
        tr_key: &str,
        tr_id: TrId,
    ) -> Result<SubscribeResponse, Error> {
        let app_key = self.auth.get_appkey();
        let app_secret = self.auth.get_appsecret();
        let personalseckey = self.auth.get_approval_key().unwrap();
        let msg_str = SubscribeRequest::new(
            app_key,
            app_secret,
            personalseckey,
            CustomerType::Personal,
            tr_key.to_string(),
            tr_id.clone(),
            TrType::Unregister,
        )
        .get_json_string();

        let url = match tr_id {
            TrId::RealtimeExec => self.exec_url.clone(),
            TrId::RealtimeOrdb => self.ordb_url.clone(),
            _ => return Err(Error::WrongTrId(tr_id, "RealtimeExec or RealtimeOrdb")),
        };

        let (ws_stream, _) = connect_async(&url).await?;
        let (mut write, mut read) = ws_stream.split();

        write.send(Message::Text(msg_str)).await?;

        let mut result = SubscribeResponse::new(false, "".to_string(), None, None);
        send_subscribe_msg(&mut read, &mut result).await?;

        if let Some(handle) = self.handles.remove(&tr_id) {
            handle.abort();
        }
        Ok(result)
    }
}

async fn send_subscribe_msg(
    read: &mut WsSplitStream,
    result: &mut SubscribeResponse,
) -> Result<(), json::Error> {
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(s)) => {
                let json_value = json::parse(&s)?;
                match json_value {
                    json::JsonValue::Object(obj) => {
                        if let Some(header) = obj.get("header") {
                            if let json::JsonValue::Object(o) = header {
                                if let Some(result_tr) = o.get("tr_id") {
                                    if &result_tr.to_string() == "PINGPONG" {
                                        continue;
                                    }
                                }
                            }
                        }
                        if let Some(v) = obj.get("body") {
                            match v {
                                json::JsonValue::Object(o) => {
                                    if let Some(s) = o.get("msg1") {
                                        let s = s.to_string();
                                        if &s == "SUBSCRIBE SUCCESS" {
                                            result.set_success(true);
                                        }
                                        result.set_msg(s);
                                    }
                                    if let Some(json::JsonValue::Object(o)) = o.get("output") {
                                        if let Some(s) = o.get("iv") {
                                            result.set_iv(Some(s.to_string()));
                                        }
                                        if let Some(s) = o.get("key") {
                                            result.set_key(Some(s.to_string()));
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        break;
    }
    Ok(())
}

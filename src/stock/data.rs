use crate::types::request::stock::subscribe::SubscribeRequest;
use crate::types::response::stock::subscribe::SubscribeResponse;
use crate::types::stream::stock::{MyExec, StreamParser};
use crate::types::{Account, CustomerType, Environment, TrId};
use crate::{auth, Error};
use std::collections::HashMap;
use websocket::{Message, OwnedMessage};

pub struct KoreaStockData {
    exec_client: websocket::ClientBuilder<'static>,
    ordb_client: websocket::ClientBuilder<'static>,
    my_exec_client: websocket::ClientBuilder<'static>,
    endpoint_url: String,
    environment: Environment,
    auth: auth::Auth,
    account: Account,
    hts_id: String,
    handles: HashMap<TrId, tokio::task::JoinHandle<()>>,
}

impl KoreaStockData {
    /// 국내 주식 실시간 시세에 관한 API
    /// [실시간시세(국내주식)](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock2-real#L_714d1437-8f62-43db-a73c-cf509d3f6aa7)
    pub fn new(
        environment: Environment,
        auth: auth::Auth,
        account: Account,
        hts_id: &str,
    ) -> Result<Self, Error> {
        let endpoint_url = match environment {
            Environment::Real => "ws://ops.koreainvestment.com:21000",
            Environment::Virtual => "ws://ops.koreainvestment.com:31000",
        }
        .to_string();
        let exec_client = websocket::ClientBuilder::new(&format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(TrId::RealtimeExec),
        ))?;
        let ordb_client = websocket::ClientBuilder::new(&format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(TrId::RealtimeOrdb),
        ))?;
        let my_exec_client = websocket::ClientBuilder::new(&format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(match environment {
                Environment::Real => TrId::RealRealtimeMyExec,
                Environment::Virtual => TrId::VirtualRealtimeMyExec,
            })
        ))?;

        Ok(Self {
            exec_client,
            ordb_client,
            my_exec_client,
            endpoint_url,
            environment,
            auth,
            account,
            hts_id: hts_id.to_string(),
            handles: HashMap::new(),
        })
    }

    /// 종목 시세 구독
    pub fn subscribe_market<T: StreamParser<R> + Send, R: Clone + Send>(
        &mut self,
        isin: &str,
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
        let msg = SubscribeRequest::new(
            app_key,
            app_secret,
            personalseckey,
            CustomerType::Personal,
            isin.to_string(),
            tr_id.clone(),
        )
        .get_json_string();
        let msg = Message::text(msg);
        let mut result = SubscribeResponse::new(false, "".to_string(), None, None);
        let mut conn = match tr_id {
            TrId::RealtimeExec => self.exec_client.connect_insecure().unwrap(),
            TrId::RealtimeOrdb => self.ordb_client.connect_insecure().unwrap(),
            _ => {
                return Err(Error::WrongTrId(tr_id, "RealtimeExec or RealtimeOrdb"));
            }
        };
        loop {
            if let Ok(msg) = {
                let _ = conn.send_message(&msg);
                conn.recv_message()
            } {
                match msg {
                    OwnedMessage::Text(s) => {
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
                                            if let Some(json::JsonValue::Object(o)) =
                                                o.get("output")
                                            {
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
            }
            break;
        }
        let handle_ref = self.handles.get(&tr_id);
        if handle_ref.is_none() || handle_ref.unwrap().is_finished() {
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
            let handle = tokio::spawn(async move {
                loop {
                    if let Ok(msg) = conn.recv_message() {
                        let tmp_msg = msg.clone();
                        match msg {
                            OwnedMessage::Text(s) => {
                                let data = T::parse(s).expect("Failed to parse message");
                                if *data.header().tr_id() == TrId::PingPong {
                                    let _ = conn.send_message(&tmp_msg);
                                } else {
                                    let _ = tx.send(data);
                                }
                            }
                            _ => {
                                error!("Get wrong data from stream={:?}", msg);
                                panic!();
                            }
                        }
                    } else {
                        error!("Failed to get message from stream");
                        panic!();
                    }
                }
            });
            self.handles.insert(tr_id, handle);

            return Ok((Some(rx), result));
        }
        Ok((None, result))
    }

    /// 체결통보 구독
    pub fn subscribe_my_exec(
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
        let msg = SubscribeRequest::new(
            app_key,
            app_secret,
            personalseckey,
            CustomerType::Personal,
            self.hts_id.clone(),
            tr_id.clone(),
        )
        .get_json_string();
        let msg = Message::text(msg);
        let mut conn = match tr_id {
            TrId::RealRealtimeMyExec => self.my_exec_client.connect_insecure().unwrap(),
            TrId::VirtualRealtimeMyExec => self.my_exec_client.connect_insecure().unwrap(),
            _ => {
                return Err(Error::WrongTrId(
                    tr_id,
                    "RealRealtimeMyExec or VirtualRealtimeMyExec",
                ));
            }
        };
        let _ = conn.send_message(&msg);
        let mut result = SubscribeResponse::new(false, "".to_string(), None, None);

        loop {
            if let Ok(msg) = conn.recv_message() {
                match msg {
                    OwnedMessage::Text(s) => {
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
                                            if let Some(json::JsonValue::Object(o)) =
                                                o.get("output")
                                            {
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
            }
            break;
        }
        let handle_ref = self.handles.get(&tr_id);
        if handle_ref.is_some() {
            handle_ref.unwrap().abort();
        }
        let (iv, key) = (
            result.iv().clone().expect("no iv"),
            result.key().clone().expect("no key"),
        );
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let handle = tokio::spawn(async move {
            loop {
                if let Ok(msg) = conn.recv_message() {
                    let tmp_msg = msg.clone();
                    match msg {
                        OwnedMessage::Text(s) => {
                            let data = MyExec::parse(s, iv.clone(), key.clone())
                                .expect("Failed to parse message");
                            if data.header().tr_id() == &TrId::PingPong {
                                let _ = conn.send_message(&tmp_msg);
                            } else {
                                let _ = tx.send(data);
                            }
                        }
                        _ => {
                            error!("Get wrong data from stream={:?}", msg);
                            panic!();
                        }
                    }
                } else {
                    error!("Failed to get message from stream");
                    panic!();
                }
            }
        });
        self.handles.insert(tr_id, handle);
        Ok((rx, result))
    }
}

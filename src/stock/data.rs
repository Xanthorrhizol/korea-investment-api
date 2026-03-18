use crate::types::request::stock::subscribe::{SubscribeRequest, TrType};
use crate::types::response::stock::subscribe::SubscribeResponse;
use crate::types::stream::stock::{Exec, MyExec, Ordb, StreamParser, exec, ordb};
use crate::types::{Account, CustomerType, Environment, TrId};
use crate::{Error, auth};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
use xan_actor::prelude::*;

type WsStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
type WsSplitStream = futures_util::stream::SplitStream<WsStream>;
type _WsSplitSink = futures_util::stream::SplitSink<WsStream, Message>;

#[allow(dead_code)]
pub struct KoreaStockData {
    krx_exec_url: String,
    krx_ordb_url: String,
    nxt_exec_url: String,
    nxt_ordb_url: String,
    union_exec_url: String,
    union_ordb_url: String,
    my_exec_url: String,
    environment: Environment,
    auth: auth::Auth,
    account: Account,
    hts_id: String,
    actor_system: ActorSystem,
    handles: HashMap<(String, TrId), tokio::task::JoinHandle<()>>,
}
impl std::fmt::Debug for KoreaStockData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KoreaStockData")
            .field("krx_exec_url", &self.krx_exec_url)
            .field("krx_ordb_url", &self.krx_ordb_url)
            .field("nxt_exec_url", &self.nxt_exec_url)
            .field("nxt_ordb_url", &self.nxt_ordb_url)
            .field("union_exec_url", &self.union_exec_url)
            .field("union_ordb_url", &self.union_ordb_url)
            .field("my_exec_url", &self.my_exec_url)
            .field("hts_id", &self.hts_id)
            .finish()
    }
}

impl KoreaStockData {
    /// 국내 주식 실시간 시세에 관한 API
    /// [실시간시세(국내주식)](https://apiportal.koreainvestment.com/apiservice-apiservice?/tryitout/H0STCNT0)
    pub async fn new(
        environment: Environment,
        auth: auth::Auth,
        account: Account,
        hts_id: &str,
    ) -> Result<Self, Error> {
        let endpoint_url = match environment {
            Environment::Real => "ws://ops.koreainvestment.com:21000",
            Environment::Virtual => "ws://ops.koreainvestment.com:31000",
        };
        let krx_exec_url = format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(TrId::RealtimeExecKrx),
        );
        let krx_ordb_url = format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(TrId::RealtimeOrdbKrx),
        );
        let my_exec_url = format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(match environment {
                Environment::Real => TrId::RealRealtimeMyExec,
                Environment::Virtual => TrId::VirtualRealtimeMyExec,
            }),
        );
        let endpoint_url = "ws://ops.koreainvestment.com:21000";
        let nxt_exec_url = format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(TrId::RealtimeExecNxt),
        );
        let nxt_ordb_url = format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(TrId::RealtimeOrdbNxt),
        );
        let union_exec_url = format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(TrId::RealtimeExecUnion),
        );
        let union_ordb_url = format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(TrId::RealtimeOrdbUnion),
        );

        let app_key = auth.get_appkey();
        let app_secret = auth.get_appsecret();
        let personalseckey = auth.get_approval_key().unwrap();

        let mut actor_system = ActorSystem::new(None);
        let krx_exec_url_data_actor = DataStreamActor::<Exec, exec::Body>::new(
            krx_exec_url.clone(),
            app_key.clone(),
            app_secret.clone(),
            personalseckey.clone(),
            environment,
        )
        .await;
        if let Err(e) = krx_exec_url_data_actor
            .register(
                &mut actor_system,
                ErrorHandling::Stop,
                Blocking::Blocking,
                None,
            )
            .await
        {
            error!("Failed to register krx_exec_url_data_actor: {}", e);
        }

        let krx_ordb_url_data_actor = DataStreamActor::<Ordb, ordb::Body>::new(
            krx_ordb_url.clone(),
            app_key.clone(),
            app_secret.clone(),
            personalseckey.clone(),
            environment,
        )
        .await;
        if let Err(e) = krx_ordb_url_data_actor
            .register(
                &mut actor_system,
                ErrorHandling::Stop,
                Blocking::Blocking,
                None,
            )
            .await
        {
            error!("Failed to register krx_ordb_url_data_actor: {}", e);
        }

        let nxt_exec_url_data_actor = DataStreamActor::<Exec, exec::Body>::new(
            nxt_exec_url.clone(),
            app_key.clone(),
            app_secret.clone(),
            personalseckey.clone(),
            environment,
        )
        .await;
        if let Err(e) = nxt_exec_url_data_actor
            .register(
                &mut actor_system,
                ErrorHandling::Stop,
                Blocking::Blocking,
                None,
            )
            .await
        {
            error!("Failed to register nxt_exec_url_data_actor: {}", e);
        }

        let nxt_ordb_url_data_actor = DataStreamActor::<Ordb, ordb::Body>::new(
            nxt_ordb_url.clone(),
            app_key.clone(),
            app_secret.clone(),
            personalseckey.clone(),
            environment,
        )
        .await;
        if let Err(e) = nxt_ordb_url_data_actor
            .register(
                &mut actor_system,
                ErrorHandling::Stop,
                Blocking::Blocking,
                None,
            )
            .await
        {
            error!("Failed to register nxt_ordb_url_data_actor: {}", e);
        }

        let union_exec_url_data_actor = DataStreamActor::<Exec, exec::Body>::new(
            union_exec_url.clone(),
            app_key.clone(),
            app_secret.clone(),
            personalseckey.clone(),
            environment,
        )
        .await;
        if let Err(e) = union_exec_url_data_actor
            .register(
                &mut actor_system,
                ErrorHandling::Stop,
                Blocking::Blocking,
                None,
            )
            .await
        {
            error!("Failed to register union_exec_url_data_actor: {}", e);
        }

        let union_ordb_url_data_actor = DataStreamActor::<Ordb, ordb::Body>::new(
            union_ordb_url.clone(),
            app_key.clone(),
            app_secret.clone(),
            personalseckey.clone(),
            environment,
        )
        .await;
        if let Err(e) = union_ordb_url_data_actor
            .register(
                &mut actor_system,
                ErrorHandling::Stop,
                Blocking::Blocking,
                None,
            )
            .await
        {
            error!("Failed to register union_ordb_url_data_actor: {}", e);
        }

        Ok(Self {
            krx_exec_url,
            krx_ordb_url,
            nxt_exec_url,
            nxt_ordb_url,
            union_exec_url,
            union_ordb_url,
            my_exec_url,
            environment,
            auth,
            account,
            actor_system,
            hts_id: hts_id.to_string(),
            handles: HashMap::new(),
        })
    }

    /// 종목 시세 구독
    pub async fn subscribe_market<
        T: StreamParser<R> + Send + Clone,
        R: Clone + Send + Sync + 'static,
    >(
        &mut self,
        tr_key: &str,
        tr_id: TrId,
    ) -> Result<
        (
            Option<tokio::sync::broadcast::Receiver<T>>,
            SubscribeResponse,
        ),
        Error,
    > {
        let url = match tr_id {
            TrId::RealtimeExecKrx => self.krx_exec_url.clone(),
            TrId::RealtimeOrdbKrx => self.krx_ordb_url.clone(),
            TrId::RealtimeExecNxt => self.nxt_exec_url.clone(),
            TrId::RealtimeOrdbNxt => self.nxt_ordb_url.clone(),
            TrId::RealtimeExecUnion => self.union_exec_url.clone(),
            TrId::RealtimeOrdbUnion => self.union_ordb_url.clone(),
            _ => {
                return Err(Error::WrongTrId(
                    tr_id,
                    "RealtimeExecXXX or RealtimeOrdbXXX",
                ));
            }
        };

        if let Ok((rx, result)) = self
            .actor_system
            .send_and_recv::<DataStreamActor<T, R>>(
                url,
                DataStreamCmdMessage::Subscribe(tr_key.to_string(), tr_id),
            )
            .await
        {
            return Ok((rx, result));
        }
        Ok((
            None,
            SubscribeResponse::new(false, "".to_string(), None, None),
        ))
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

        crate::wait(self.environment).await;
        write.send(Message::Text(msg_str)).await?;
        crate::update_last_call();

        let mut result = SubscribeResponse::new(false, "".to_string(), None, None);
        recv_subscribe_response(&mut read, &mut result).await?;

        if let Some(handle) = self.handles.remove(&(String::default(), tr_id)) {
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
        self.handles.insert((String::default(), tr_id), handle);
        Ok((rx, result))
    }

    /// 종목 시세 구독 해체
    pub async fn unsubscribe_market(
        &mut self,
        tr_key: &str,
        tr_id: TrId,
    ) -> Result<SubscribeResponse, Error> {
        let url = match tr_id {
            TrId::RealtimeExecKrx => self.krx_exec_url.clone(),
            TrId::RealtimeOrdbKrx => self.krx_ordb_url.clone(),
            TrId::RealtimeExecNxt => self.nxt_exec_url.clone(),
            TrId::RealtimeOrdbNxt => self.nxt_ordb_url.clone(),
            TrId::RealtimeExecUnion => self.union_exec_url.clone(),
            TrId::RealtimeOrdbUnion => self.union_ordb_url.clone(),
            _ => {
                return Err(Error::WrongTrId(
                    tr_id,
                    "RealtimeExecXXX or RealtimeOrdbXXX",
                ));
            }
        };

        match tr_id {
            TrId::RealtimeExecKrx => {
                if let Ok((_rx, result)) = self
                    .actor_system
                    .send_and_recv::<DataStreamActor<Exec, exec::Body>>(
                        url,
                        DataStreamCmdMessage::Unsubscribe(tr_key.to_string(), tr_id),
                    )
                    .await
                {
                    return Ok(result);
                }
            }
            TrId::RealtimeOrdbKrx => {
                if let Ok((_rx, result)) = self
                    .actor_system
                    .send_and_recv::<DataStreamActor<Ordb, ordb::Body>>(
                        url,
                        DataStreamCmdMessage::Unsubscribe(tr_key.to_string(), tr_id),
                    )
                    .await
                {
                    return Ok(result);
                }
            }
            TrId::RealtimeExecNxt => {
                if let Ok((_rx, result)) = self
                    .actor_system
                    .send_and_recv::<DataStreamActor<Exec, exec::Body>>(
                        url,
                        DataStreamCmdMessage::Unsubscribe(tr_key.to_string(), tr_id),
                    )
                    .await
                {
                    return Ok(result);
                }
            }
            TrId::RealtimeOrdbNxt => {
                if let Ok((_rx, result)) = self
                    .actor_system
                    .send_and_recv::<DataStreamActor<Ordb, ordb::Body>>(
                        url,
                        DataStreamCmdMessage::Unsubscribe(tr_key.to_string(), tr_id),
                    )
                    .await
                {
                    return Ok(result);
                }
            }
            TrId::RealtimeExecUnion => {
                if let Ok((_rx, result)) = self
                    .actor_system
                    .send_and_recv::<DataStreamActor<Exec, exec::Body>>(
                        url,
                        DataStreamCmdMessage::Unsubscribe(tr_key.to_string(), tr_id),
                    )
                    .await
                {
                    return Ok(result);
                }
            }
            TrId::RealtimeOrdbUnion => {
                if let Ok((_rx, result)) = self
                    .actor_system
                    .send_and_recv::<DataStreamActor<Ordb, ordb::Body>>(
                        url,
                        DataStreamCmdMessage::Unsubscribe(tr_key.to_string(), tr_id),
                    )
                    .await
                {
                    return Ok(result);
                }
            }
            _ => {}
        }
        Ok(SubscribeResponse::new(false, "".to_string(), None, None))
    }
}

async fn recv_subscribe_response(
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

impl Drop for KoreaStockData {
    fn drop(&mut self) {
        let keys: Vec<_> = self.handles.keys().cloned().collect();
        let handle = tokio::runtime::Handle::current();
        for (tr_key, tr_id) in keys {
            let _ = tokio::task::block_in_place(|| {
                handle.block_on(self.unsubscribe_market(&tr_key, tr_id))
            });
        }
    }
}

#[derive(Debug, Clone)]
enum DataStreamCmdMessage {
    Subscribe(String, TrId), // tr_key, tr_id
    Unsubscribe(String, TrId),
}

struct DataStreamActor<T: StreamParser<R> + Send, R: Clone + Send> {
    url: String,
    cmd_tx: tokio::sync::mpsc::UnboundedSender<(
        Arc<DataStreamCmdMessage>,
        tokio::sync::oneshot::Sender<(
            Option<tokio::sync::broadcast::Receiver<T>>,
            SubscribeResponse,
        )>,
    )>,
    _marker: std::marker::PhantomData<R>,
}

impl<T, R> DataStreamActor<T, R>
where
    T: StreamParser<R> + Clone + Send,
    R: Clone + Send + Sync + 'static,
{
    pub async fn new(
        url: String,
        app_key: String,
        app_secret: String,
        personalseckey: String,
        environment: Environment,
    ) -> DataStreamActor<T, R> {
        let (ws_stream, _) = connect_async(&url).await.unwrap();
        let (mut write, mut read) = ws_stream.split();

        let (tx, rx) = tokio::sync::broadcast::channel(4096);
        let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::unbounded_channel::<(
            Arc<DataStreamCmdMessage>,
            tokio::sync::oneshot::Sender<(
                Option<tokio::sync::broadcast::Receiver<T>>,
                SubscribeResponse,
            )>,
        )>();
        let app_key_clone = app_key.clone();
        let app_secret_clone = app_secret.clone();
        let personalseckey_clone = personalseckey.clone();
        let rx_clone = rx.resubscribe();
        tokio::spawn(async move {
            let rx = rx_clone;
            loop {
                let app_key = app_key_clone.clone();
                let app_secret = app_secret_clone.clone();
                let personalseckey = personalseckey_clone.clone();
                tokio::select! {
                    msg = read.next() => {
                        match msg {
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
                    Some(cmd) = cmd_rx.recv() => {
                        let msg = cmd.0.as_ref();
                        let result_tx = cmd.1;
                        match msg {
                            DataStreamCmdMessage::Subscribe(tr_key, tr_id) => {
                                let msg_str = SubscribeRequest::new(
                                    app_key.clone(),
                                    app_secret.clone(),
                                    personalseckey.clone(),
                                    CustomerType::Personal,
                                    tr_key.clone(),
                                    tr_id.clone(),
                                    TrType::Register,
                                )
                                .get_json_string();
                                crate::wait(environment).await;
                                let _ = write.send(Message::Text(msg_str)).await;
                                crate::update_last_call();

                                let mut result = SubscribeResponse::new(false, "".to_string(), None, None);
                                if let Err(e) = recv_subscribe_response(&mut read, &mut result).await {
                                    error!("Failed to subscribe: {}", e);
                                }
                                result_tx.send((Some(rx.resubscribe()), result)).expect("Failed to send result");
                            }
                            DataStreamCmdMessage::Unsubscribe(tr_key, tr_id) => {
                                let msg_str = SubscribeRequest::new(
                                    app_key.clone(),
                                    app_secret.clone(),
                                    personalseckey.clone(),
                                    CustomerType::Personal,
                                    tr_key.clone(),
                                    tr_id.clone(),
                                    TrType::Unregister,
                                )
                                .get_json_string();
                                crate::wait(environment).await;
                                let _ = write.send(Message::Text(msg_str)).await;
                                crate::update_last_call();

                                let mut result = SubscribeResponse::new(false, "".to_string(), None, None);
                                if let Err(e) = recv_subscribe_response(&mut read, &mut result).await {
                                    error!("Failed to unsubscribe: {}", e);
                                }
                                result_tx.send((None, result)).expect("Failed to send result");
                            }
                        }
                    }
                }
            }
        });

        DataStreamActor {
            url,
            cmd_tx,
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T, R> Actor for DataStreamActor<T, R>
where
    T: StreamParser<R> + Clone + Send,
    R: Clone + Sync + Send + 'static,
{
    type Message = DataStreamCmdMessage;
    type Result = (
        Option<tokio::sync::broadcast::Receiver<T>>,
        SubscribeResponse,
    );
    type Error = Error;

    fn address(&self) -> &str {
        &self.url
    }

    async fn handle(&mut self, msg: Arc<Self::Message>) -> Result<Self::Result, Self::Error> {
        let (result_tx, result_rx) = tokio::sync::oneshot::channel();
        let _ = self.cmd_tx.send((msg, result_tx));
        Ok(result_rx.await.expect("Failed to get result"))
    }
}

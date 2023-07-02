use crate::types::{CustomerType, Exec, MyExec, Ordb, Subscribe, SubscribeResult, TrId};
use crate::{auth, Account, Environment, Error};
use websocket::{Message, OwnedMessage};

pub struct KoreaStockData {
    exec_conn: websocket::client::sync::Client<std::net::TcpStream>,
    ordb_conn: websocket::client::sync::Client<std::net::TcpStream>,
    my_exec_conn: websocket::client::sync::Client<std::net::TcpStream>,
    endpoint_url: String,
    environment: Environment,
    auth: auth::Auth,
    account: Account,
    usehash: bool,
    hts_id: String,
}

impl KoreaStockData {
    /// 국내 주식 실시간 시세에 관한 API
    /// [국내주식시세](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock-quotations#L_07802512-4f49-4486-91b4-1050b6f5dc9d)
    pub fn new(
        environment: Environment,
        auth: auth::Auth,
        account: Account,
        usehash: bool,
        hts_id: String,
    ) -> Result<Self, Error> {
        let endpoint_url = match environment {
            Environment::Real => "ws://ops.koreainvestment.com:21000",
            Environment::Virtual => "ws://ops.koreainvestment.com:31000",
        }
        .to_string();
        let mut exec_client = websocket::ClientBuilder::new(&format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(TrId::RealtimeExec),
        ))?;
        let mut ordb_client = websocket::ClientBuilder::new(&format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(TrId::RealtimeOrdb),
        ))?;
        let mut my_exec_client = websocket::ClientBuilder::new(&format!(
            "{}/tryitout/{}",
            endpoint_url,
            Into::<String>::into(match environment {
                Environment::Real => TrId::RealRealtimeMyExec,
                Environment::Virtual => TrId::VirtualRealtimeMyExec,
            })
        ))?;
        let exec_conn = exec_client.connect_insecure().unwrap();
        let ordb_conn = ordb_client.connect_insecure().unwrap();
        let my_exec_conn = my_exec_client.connect_insecure().unwrap();

        Ok(Self {
            exec_conn,
            ordb_conn,
            my_exec_conn,
            endpoint_url,
            environment,
            auth,
            account,
            usehash,
            hts_id,
        })
    }

    pub fn subscribe_market(
        &mut self,
        isin: String,
        tr_id: TrId,
    ) -> Result<SubscribeResult, Error> {
        let app_key = self.auth.get_appkey();
        let app_secret = self.auth.get_appsecret();
        let personalseckey = self.auth.get_approval_key().unwrap();
        let msg = Subscribe::new(
            app_key,
            app_secret,
            personalseckey,
            CustomerType::Personal,
            isin,
            tr_id.clone(),
        )
        .get_json_string();
        let msg = Message::text(msg);
        let _ = self.exec_conn.send_message(&msg);
        let mut result = SubscribeResult::new(false, "".to_string(), None, None);
        if let Ok(msg) = if tr_id == TrId::RealtimeExec {
            self.exec_conn.recv_message()
        } else if tr_id == TrId::RealtimeOrdb {
            self.ordb_conn.recv_message()
        } else {
            Err(Error::WrongTrId(tr_id, "RealtimeExec, RealtimeOrdb"))?
        } {
            match msg {
                OwnedMessage::Text(s) => {
                    let json_value = json::parse(&s)?;
                    match json_value {
                        json::JsonValue::Object(obj) => {
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
        }
        Ok(result)
    }

    pub fn subscribe_my_exec(&mut self) -> Result<SubscribeResult, Error> {
        let app_key = self.auth.get_appkey();
        let app_secret = self.auth.get_appsecret();
        let personalseckey = self.auth.get_approval_key().unwrap();
        let tr_id = match self.environment {
            Environment::Real => TrId::RealRealtimeMyExec,
            Environment::Virtual => TrId::VirtualRealtimeMyExec,
        };
        let msg = Subscribe::new(
            app_key,
            app_secret,
            personalseckey,
            CustomerType::Personal,
            self.hts_id.clone(),
            tr_id.clone(),
        )
        .get_json_string();
        let msg = Message::text(msg);
        let _ = self.my_exec_conn.send_message(&msg);
        let mut result = SubscribeResult::new(false, "".to_string(), None, None);

        if let Ok(msg) = self.my_exec_conn.recv_message() {
            match msg {
                OwnedMessage::Text(s) => {
                    let json_value = json::parse(&s)?;
                    match json_value {
                        json::JsonValue::Object(obj) => {
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
        }
        Ok(result)
    }

    /// 국내주식 실시간체결가[실시간-003]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock2-real#L_714d1437-8f62-43db-a73c-cf509d3f6aa7)
    pub fn exec_recv(&mut self) -> Result<Exec, Error> {
        if let Ok(msg) = self.exec_conn.recv_message() {
            let tmp_msg = msg.clone();
            match msg {
                OwnedMessage::Text(s) => {
                    let exec = Exec::parse(s)?;
                    if exec.header().tr_id() == &TrId::PingPong {
                        let _ = self.exec_conn.send_message(&tmp_msg);
                    }
                    Ok(exec)
                }
                _ => {
                    return Err(Error::InvalidData);
                }
            }
        } else {
            Err(Error::InvalidData)
        }
    }

    /// 국내주식 실시간호가[실시간-004]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock2-real#L_9cda726b-6f0b-48b5-8369-6d66bea05a2a)
    pub fn ordb_recv(&mut self) -> Result<Ordb, Error> {
        if let Ok(msg) = self.ordb_conn.recv_message() {
            let tmp_msg = msg.clone();
            match msg {
                OwnedMessage::Text(s) => {
                    let ordb = Ordb::parse(s)?;
                    if ordb.header().tr_id() == &TrId::PingPong {
                        let _ = self.ordb_conn.send_message(&tmp_msg);
                    }
                    Ok(ordb)
                }
                _ => {
                    return Err(Error::InvalidData);
                }
            }
        } else {
            Err(Error::InvalidData)
        }
    }

    /// 국내주식 실시간체결통보[실시간-005]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock2-real#L_1e3c056d-1b42-461c-b8fb-631bb48e1ee2)
    pub fn my_exec_recv(&mut self, iv: String, key: String) -> Result<MyExec, Error> {
        if let Ok(msg) = self.my_exec_conn.recv_message() {
            let tmp_msg = msg.clone();
            match msg {
                OwnedMessage::Text(s) => {
                    let my_exec = MyExec::parse(s, iv, key)?;
                    if my_exec.header().tr_id() == &TrId::PingPong {
                        let _ = self.my_exec_conn.send_message(&tmp_msg);
                    }
                    Ok(my_exec)
                }
                _ => {
                    return Err(Error::InvalidData);
                }
            }
        } else {
            Err(Error::InvalidData)
        }
    }
}

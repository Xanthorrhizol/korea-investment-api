use crate::types::Exec;
use crate::{auth, Account, Environment, Error};
use websocket::OwnedMessage;

#[derive(Clone)]
pub struct KoreaStockData<'a> {
    client: websocket::ClientBuilder<'a>,
    endpoint_url: String,
    environment: Environment,
    auth: auth::Auth,
    account: Account,
    usehash: bool,
}

impl<'a> KoreaStockData<'a> {
    /// 국내 주식 실시간 시세에 관한 API
    /// [국내주식시세](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock-quotations#L_07802512-4f49-4486-91b4-1050b6f5dc9d)
    pub fn new(
        environment: Environment,
        auth: auth::Auth,
        account: Account,
        usehash: bool,
    ) -> Result<Self, Error> {
        let endpoint_url = match environment {
            Environment::Real => "ws://ops.koreainvestment.com:21000",
            Environment::Virtual => "ws://ops.koreainvestment.com:31000",
        }
        .to_string();
        let client = websocket::ClientBuilder::new(&endpoint_url)?;

        Ok(Self {
            client,
            endpoint_url,
            environment,
            auth,
            account,
            usehash,
        })
    }

    /// 국내주식 실시간체결가[실시간-003]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock2-real#L_714d1437-8f62-43db-a73c-cf509d3f6aa7)
    pub fn exec_stream(
        &mut self,
        tx: tokio::sync::mpsc::UnboundedSender<Result<Exec, Error>>,
    ) -> Result<(), Error> {
        let mut conn = self.client.connect_insecure().unwrap();
        while let Ok(msg) = conn.recv_message() {
            match msg {
                OwnedMessage::Text(s) => {
                    let json_value = json::parse(&s)?;
                    let exec = Exec::parse(json_value)?;
                    let _ = tx.send(Ok(exec));
                }
                _ => {
                    return Err(Error::InvalidData);
                }
            }
        }
        Ok(())
    }

    // 국내주식 실시간호가[실시간-004]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock2-real#L_9cda726b-6f0b-48b5-8369-6d66bea05a2a)

    // 국내주식 실시간체결통보[실시간-005]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock2-real#L_1e3c056d-1b42-461c-b8fb-631bb48e1ee2)
}

use crate::types::stock;
use crate::{auth, Account, Environment, Error};
use websocket::native_tls::{TlsConnector, TlsStream};

pub struct Korea {
    client: reqwest::Client,
    wsclient: websocket::client::sync::Client<TlsStream<std::net::TcpStream>>,
    endpoint_url: String,
    environment: Environment,
    auth: &'static auth::Auth,
    account: Account,
}

impl Korea {
    /// 국내 주식 주문/시세에 관한 API
    /// [국내주식주문](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_aade4c72-5fb7-418a-9ff2-254b4d5f0ceb)
    /// [국내주식시세](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock-quotations#L_07802512-4f49-4486-91b4-1050b6f5dc9d)
    pub fn new(
        client: &reqwest::Client,
        endpoint_url: &str,
        environment: Environment,
        auth: &'static auth::Auth,
        account: Account,
    ) -> Result<Self, Error> {
        let wsclient = websocket::ClientBuilder::new(endpoint_url)?
            .connect_secure(Some(TlsConnector::new()?))?;
        Ok(Self {
            client: client.clone(),
            wsclient,
            endpoint_url: endpoint_url.to_string(),
            environment,
            auth,
            account,
        })
    }

    /// 주식주문(현금)[v1_국내주식-001]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_aade4c72-5fb7-418a-9ff2-254b4d5f0ceb)
    pub async fn order_cash(
        &self,
        order_division: stock::OrderDivision,
        order_direction: stock::Direction,
        pdno: String,
        qty: stock::Quantity,
        price: stock::Price,
    ) -> Result<stock::ResponseBody, Error> {
        let request = stock::RequestBody::new(
            self.account.cano.clone(),
            self.account.acnt_prdt_cd.clone(),
            pdno,
            order_division,
            qty,
            price,
        )
        .get_json_string();
        let tr_id = match self.environment {
            Environment::Real => match order_direction {
                stock::Direction::Bid => Into::<String>::into(stock::TrId::RealStockCashBidOrder),
                stock::Direction::Ask => Into::<String>::into(stock::TrId::RealStockCashAskOrder),
            },
            Environment::Virtual => match order_direction {
                stock::Direction::Bid => {
                    Into::<String>::into(stock::TrId::VirtualStockCashBidOrder)
                }
                stock::Direction::Ask => {
                    Into::<String>::into(stock::TrId::VirtualStockCashAskOrder)
                }
            },
        };
        let hash = self.auth.get_hash(request.clone()).await?;
        Ok(self
            .client
            .post(format!(
                "{}/uapi/domestic-stock/v1/trading/order-cash",
                self.endpoint_url
            ))
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                match self.auth.get_token() {
                    Some(token) => token,
                    None => {
                        return Err(Error::AuthInitFailed("token"));
                    }
                },
            )
            .header("appkey", self.auth.get_appkey())
            .header("appsecret", self.auth.get_appsecret())
            .header("tr_id", tr_id)
            .header("hashkey", hash)
            .body(request)
            .send()
            .await?
            .json::<stock::ResponseBody>()
            .await?)
    }
    // TODO: 주식주문(신용)
    // 주식주문(정정취소)
    // 주식정정취소가능주문조회
    // TODO: 주식일별주문체결조회
    // 주식잔고조회
    // 매수가능조회
}

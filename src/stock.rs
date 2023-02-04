use crate::types::stock;
use crate::{auth, Account, Environment, Error};
use websocket::native_tls::{TlsConnector, TlsStream};

pub struct Korea {
    client: reqwest::Client,
    wsclient: websocket::client::sync::Client<TlsStream<std::net::TcpStream>>,
    endpoint_url: String,
    environment: Environment,
    auth: auth::Auth,
    account: Account,
    usehash: bool,
}

impl Korea {
    /// 국내 주식 주문/시세에 관한 API
    /// [국내주식주문](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_aade4c72-5fb7-418a-9ff2-254b4d5f0ceb)
    /// [국내주식시세](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock-quotations#L_07802512-4f49-4486-91b4-1050b6f5dc9d)
    pub fn new(
        client: &reqwest::Client,
        endpoint_url: &str,
        environment: Environment,
        auth: auth::Auth,
        account: Account,
        usehash: bool,
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
            usehash,
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
        match self.usehash {
            true => {
                self.order_cash_w_hash(order_division, order_direction, pdno, qty, price)
                    .await
            }
            false => {
                self.order_cash_wo_hash(order_division, order_direction, pdno, qty, price)
                    .await
            }
        }
    }
    pub async fn order_cash_wo_hash(
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
            .body(request)
            .send()
            .await?
            .json::<stock::ResponseBody>()
            .await?)
    }
    pub async fn order_cash_w_hash(
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

    /// 주식주문(신용)[v1_국내주식-002]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_f5769e4a-24d5-44f9-a2d8-232d45abf988)

    /// 주식주문(정정취소)[v1_국내주식-003]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_4bfdfb2b-34a7-43f6-935a-e637724f960a)

    /// 주식정정취소가능주문조회[v1_국내주식-004]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_d4537e9c-73f7-414c-9fb0-4eae3bc397d0)

    /// 주식일별주문체결조회[v1_국내주식-005]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_bc51f9f7-146f-4971-a5ae-ebd574acec12)

    /// 주식잔고조회[v1_국내주식-006]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_66c61080-674f-4c91-a0cc-db5e64e9a5e6)

    /// 매수가능조회[v1_국내주식-007]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_806e407c-3082-44c0-9d71-e8534db5ad54)
}

use crate::types::{
    request, response, CorrectionClass, Direction, OrderClass, Price, Quantity, TrId,
};
use crate::{auth, Account, Environment, Error};

#[derive(Clone)]
pub struct Korea {
    client: reqwest::Client,
    endpoint_url: String,
    environment: Environment,
    auth: auth::Auth,
    account: Account,
}

impl Korea {
    /// 국내 주식 주문에 관한 API
    /// [국내주식주문](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_aade4c72-5fb7-418a-9ff2-254b4d5f0ceb)
    pub fn new(
        client: &reqwest::Client,
        environment: Environment,
        auth: auth::Auth,
        account: Account,
    ) -> Result<Self, Error> {
        let endpoint_url = match environment {
            Environment::Real => "https://openapi.koreainvestment.com:9443",
            Environment::Virtual => "https://openapivts.koreainvestment.com:29443",
        }
        .to_string();
        Ok(Self {
            client: client.clone(),
            endpoint_url,
            environment,
            auth,
            account,
        })
    }

    /// 주식주문(현금)[v1_국내주식-001]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_aade4c72-5fb7-418a-9ff2-254b4d5f0ceb)
    pub async fn order_cash(
        &self,
        order_division: OrderClass,
        order_direction: Direction,
        pdno: String,
        qty: Quantity,
        price: Price,
    ) -> Result<response::order::Body::Order, Error> {
        let request = request::order::Body::Order::new(
            self.account.cano.clone(),
            self.account.acnt_prdt_cd.clone(),
            pdno,
            order_division,
            qty,
            price,
        )
        .get_json_string();
        let tr_id: String = match self.environment {
            Environment::Real => match order_direction {
                Direction::Bid => TrId::RealStockCashBidOrder.into(),
                Direction::Ask => TrId::RealStockCashAskOrder.into(),
            },
            Environment::Virtual => match order_direction {
                Direction::Bid => TrId::VirtualStockCashBidOrder.into(),
                Direction::Ask => TrId::VirtualStockCashAskOrder.into(),
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
                    Some(token) => format!("Bearer {}", token),
                    None => {
                        return Err(Error::AuthInitFailed("token"));
                    }
                },
            )
            .header("appkey", self.auth.get_appkey())
            .header("appsecret", self.auth.get_appsecret())
            .header("tr_id", tr_id)
            .header("hashkey", hash)
            .header("custtype", "P")
            .body(request)
            .send()
            .await?
            .json::<response::order::Body::Order>()
            .await?)
    }

    // TODO: 주식주문(신용)[v1_국내주식-002]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_f5769e4a-24d5-44f9-a2d8-232d45abf988)

    /// 주식주문(정정취소)[v1_국내주식-003] TODO: test
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_4bfdfb2b-34a7-43f6-935a-e637724f960a)
    pub async fn correct(
        &self,
        order_division: OrderClass,
        krx_fwdg_ord_orgno: String,
        orgn_odno: String,
        rvse_cncl_dvsn_cd: CorrectionClass,
        qty_all_ord_yn: bool,
        qty: Quantity,
        price: Price,
    ) -> Result<response::order::Body::Order, Error> {
        let request = request::order::Body::Correction::new(
            self.account.cano.clone(),
            self.account.acnt_prdt_cd.clone(),
            krx_fwdg_ord_orgno,
            orgn_odno,
            order_division,
            rvse_cncl_dvsn_cd,
            qty,
            price,
            qty_all_ord_yn,
        )
        .get_json_string();
        let tr_id: String = match self.environment {
            Environment::Real => TrId::RealStockCorrection.into(),
            Environment::Virtual => TrId::VirtualStockCorrection.into(),
        };
        let hash = self.auth.get_hash(request.clone()).await?;
        Ok(self
            .client
            .post(format!(
                "{}/uapi/domestic-stock/v1/trading/order-rvsecncl",
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
            .json::<response::order::Body::Order>()
            .await?)
    }

    // TODO: 주식정정취소가능주문조회[v1_국내주식-004]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_d4537e9c-73f7-414c-9fb0-4eae3bc397d0)

    // TODO: 주식일별주문체결조회[v1_국내주식-005]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_bc51f9f7-146f-4971-a5ae-ebd574acec12)

    // TODO: 주식잔고조회[v1_국내주식-006]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_66c61080-674f-4c91-a0cc-db5e64e9a5e6)

    // TODO: 매수가능조회[v1_국내주식-007]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_806e407c-3082-44c0-9d71-e8534db5ad54)
}

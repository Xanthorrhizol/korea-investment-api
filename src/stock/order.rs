use crate::types::{
    request, response, Account, CorrectionClass, Direction, Environment, OrderClass, Price,
    Quantity, TrId,
};
use crate::{auth, Error};

#[derive(Clone, Debug)]
pub struct Korea {
    client: reqwest::Client,
    endpoint_url: String,
    environment: Environment,
    auth: auth::Auth,
    account: Account,
}

impl Korea {
    /// 국내 주식 주문에 관한 API
    /// [국내주식주문](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/order-cash)
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
    /// [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/order-cash)
    pub async fn order_cash(
        &self,
        order_division: OrderClass,
        order_direction: Direction,
        pdno: &str,
        qty: Quantity,
        price: Price,
    ) -> Result<response::stock::order::Body::Order, Error> {
        let request = request::stock::order::Body::Order::new(
            self.account.cano.clone(),
            self.account.acnt_prdt_cd.clone(),
            pdno.to_string(),
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
            .json::<response::stock::order::Body::Order>()
            .await?)
    }

    // TODO: 주식주문(신용)[v1_국내주식-002]
    // [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/order-credit)

    /// 주식주문(정정취소)[v1_국내주식-003] TODO: test
    /// [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/order-rvsecncl)
    pub async fn correct(
        &self,
        order_division: OrderClass,
        krx_fwdg_ord_orgno: &str,
        orgn_odno: &str,
        rvse_cncl_dvsn_cd: CorrectionClass,
        qty_all_ord_yn: bool,
        qty: Quantity,
        price: Price,
    ) -> Result<response::stock::order::Body::Order, Error> {
        let request = request::stock::order::Body::Correction::new(
            self.account.cano.clone(),
            self.account.acnt_prdt_cd.clone(),
            krx_fwdg_ord_orgno.to_string(),
            orgn_odno.to_string(),
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
            .json::<response::stock::order::Body::Order>()
            .await?)
    }

    // TODO: 주식정정취소가능주문조회[v1_국내주식-004]
    // [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl)

    // TODO: 주식일별주문체결조회[v1_국내주식-005]
    // [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/inquire-daily-ccld)

    // TODO: 주식잔고조회[v1_국내주식-006]
    // [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/inquire-balance)

    // TODO: 매수가능조회[v1_국내주식-007]
    // [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/inquire-psbl-order)
}

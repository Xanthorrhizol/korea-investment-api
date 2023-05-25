use crate::types::{
    request, response, CorrectionDivision, Direction, OrderDivision, Price, Quantity, TrId,
};
use crate::{auth, Account, Environment, Error};
// use websocket::native_tls::{TlsConnector, TlsStream};

pub struct Korea<'a> {
    client: reqwest::Client,
    wsclient: websocket::ClientBuilder<'a>,
    endpoint_url: String,
    wsendpoint_url: websocket::url::Url,
    environment: Environment,
    auth: auth::Auth,
    account: Account,
    usehash: bool,
}

impl<'a> Korea<'a> {
    /// 국내 주식 주문/시세에 관한 API
    /// [국내주식주문](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_aade4c72-5fb7-418a-9ff2-254b4d5f0ceb)
    /// [국내주식시세](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock-quotations#L_07802512-4f49-4486-91b4-1050b6f5dc9d)
    pub fn new(
        client: &reqwest::Client,
        endpoint_url: String,
        wsendpoint_url: String,
        environment: Environment,
        auth: auth::Auth,
        account: Account,
        usehash: bool,
    ) -> Result<Self, Error> {
        let wsclient = websocket::ClientBuilder::new(&wsendpoint_url)?;
        let wsendpoint_url = websocket::url::Url::parse(&wsendpoint_url)?;

        Ok(Self {
            client: client.clone(),
            wsclient,
            endpoint_url: endpoint_url.to_string(),
            wsendpoint_url,
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
        order_division: OrderDivision,
        order_direction: Direction,
        pdno: String,
        qty: Quantity,
        price: Price,
    ) -> Result<response::stock::Body::Order, Error> {
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
        order_division: OrderDivision,
        order_direction: Direction,
        pdno: String,
        qty: Quantity,
        price: Price,
    ) -> Result<response::stock::Body::Order, Error> {
        let request = request::stock::Body::Order::new(
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
            .json::<response::stock::Body::Order>()
            .await?)
    }
    pub async fn order_cash_w_hash(
        &self,
        order_division: OrderDivision,
        order_direction: Direction,
        pdno: String,
        qty: Quantity,
        price: Price,
    ) -> Result<response::stock::Body::Order, Error> {
        let request = request::stock::Body::Order::new(
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
            .json::<response::stock::Body::Order>()
            .await?)
    }

    // 주식주문(신용)[v1_국내주식-002]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_f5769e4a-24d5-44f9-a2d8-232d45abf988)

    // 주식주문(정정취소)[v1_국내주식-003]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_4bfdfb2b-34a7-43f6-935a-e637724f960a)
    pub async fn correct(
        &self,
        order_division: OrderDivision,
        krx_fwdg_ord_orgno: String,
        orgn_odno: String,
        rvse_cncl_dvsn_cd: CorrectionDivision,
        qty_all_ord_yn: bool,
        qty: Quantity,
        price: Price,
    ) -> Result<response::stock::Body::Order, Error> {
        match self.usehash {
            true => {
                self.correct_w_hash(
                    order_division,
                    krx_fwdg_ord_orgno,
                    orgn_odno,
                    rvse_cncl_dvsn_cd,
                    qty_all_ord_yn,
                    qty,
                    price,
                )
                .await
            }
            false => {
                self.correct_wo_hash(
                    order_division,
                    krx_fwdg_ord_orgno,
                    orgn_odno,
                    rvse_cncl_dvsn_cd,
                    qty_all_ord_yn,
                    qty,
                    price,
                )
                .await
            }
        }
    }
    pub async fn correct_wo_hash(
        &self,
        order_division: OrderDivision,
        krx_fwdg_ord_orgno: String,
        orgn_odno: String,
        rvse_cncl_dvsn_cd: CorrectionDivision,
        qty_all_ord_yn: bool,
        qty: Quantity,
        price: Price,
    ) -> Result<response::stock::Body::Order, Error> {
        let request = request::stock::Body::Correction::new(
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
            .body(request)
            .send()
            .await?
            .json::<response::stock::Body::Order>()
            .await?)
    }
    pub async fn correct_w_hash(
        &self,
        order_division: OrderDivision,
        krx_fwdg_ord_orgno: String,
        orgn_odno: String,
        rvse_cncl_dvsn_cd: CorrectionDivision,
        qty_all_ord_yn: bool,
        qty: Quantity,
        price: Price,
    ) -> Result<response::stock::Body::Order, Error> {
        let request = request::stock::Body::Correction::new(
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
            .json::<response::stock::Body::Order>()
            .await?)
    }

    // 주식정정취소가능주문조회[v1_국내주식-004]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_d4537e9c-73f7-414c-9fb0-4eae3bc397d0)

    // 주식일별주문체결조회[v1_국내주식-005]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_bc51f9f7-146f-4971-a5ae-ebd574acec12)

    // 주식잔고조회[v1_국내주식-006]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_66c61080-674f-4c91-a0cc-db5e64e9a5e6)

    // 매수가능조회[v1_국내주식-007]
    // [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_806e407c-3082-44c0-9d71-e8534db5ad54)
    //
    //
    //        let stream = self
    //          .wsclient
    //          .connect_secure(Some(TlsConnector::new()?))?
    //          .into_stream()
    //          .0;
}

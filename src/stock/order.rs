use crate::types::{
    request, response, Account, CorrectionClass, CreditType, Direction, Environment, OrderClass,
    Price, Quantity, TargetExchange, TrId,
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
        excg_id_dvsn_cd: Option<TargetExchange>,
    ) -> Result<response::stock::order::Body::Order, Error> {
        let request = request::stock::order::Body::Order::new(
            self.account.cano.clone(),
            self.account.acnt_prdt_cd.clone(),
            pdno.to_string(),
            order_division,
            qty,
            price,
            excg_id_dvsn_cd,
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
        crate::wait(self.environment).await;
        let result = self
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
            .await?;
        crate::update_last_call();
        Ok(result)
    }

    /// 주식주문(신용)[v1_국내주식-002]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/order-credit)
    pub async fn order_credit(
        &self,
        order_division: OrderClass,
        order_direction: Direction,
        pdno: &str,
        qty: Quantity,
        price: Price,
        excg_id_dvsn_cd: Option<TargetExchange>,
        credit_type: CreditType,
    ) -> Result<response::stock::order::Body::Order, Error> {
        let request = request::stock::order::Body::CreditOrder::new(
            self.account.cano.clone(),
            self.account.acnt_prdt_cd.clone(),
            pdno.to_string(),
            credit_type,
            chrono::Utc::now()
                .with_timezone(&chrono_tz::Asia::Seoul)
                .format("%Y%m%d")
                .to_string(),
            order_division,
            qty,
            price,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            excg_id_dvsn_cd.unwrap_or_default(),
            None,
        )
        .get_json_string();
        let tr_id: String = match self.environment {
            Environment::Real => match order_direction {
                Direction::Bid => TrId::RealStockCreditBidOrder.into(),
                Direction::Ask => TrId::RealStockCreditAskOrder.into(),
            },
            Environment::Virtual => {
                return Err(Error::NotSupported("Virtual Stock Credit Order"));
            }
        };
        let hash = self.auth.get_hash(request.clone()).await?;
        crate::wait(self.environment).await;
        let result = self
            .client
            .post(format!(
                "{}/uapi/domestic-stock/v1/trading/order-credit",
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
            .await?;
        crate::update_last_call();
        Ok(result)
    }

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
        excg_id_dvsn_cd: Option<TargetExchange>,
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
            excg_id_dvsn_cd,
        )
        .get_json_string();
        let tr_id: String = match self.environment {
            Environment::Real => TrId::RealStockCorrection.into(),
            Environment::Virtual => TrId::VirtualStockCorrection.into(),
        };
        let hash = self.auth.get_hash(request.clone()).await?;
        crate::wait(self.environment).await;
        let result = self
            .client
            .post(format!(
                "{}/uapi/domestic-stock/v1/trading/order-rvsecncl",
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
            .body(request)
            .send()
            .await?
            .json::<response::stock::order::Body::Order>()
            .await?;
        crate::update_last_call();
        Ok(result)
    }

    // TODO: 주식정정취소가능주문조회[v1_국내주식-004]
    // [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl)

    // TODO: 주식일별주문체결조회[v1_국내주식-005]
    // [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/inquire-daily-ccld)

    /// 주식잔고조회[v1_국내주식-006]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/inquire-balance)
    ///
    /// 실전계좌: 최대 50건, 모의계좌: 최대 20건 / 초과분은 `ctx_area_fk100`/`ctx_area_nk100`으로 연속조회
    pub async fn inquire_balance(
        &self,
        afhr_flpr_yn: Option<String>,
        inqr_dvsn: Option<String>,
        prcs_dvsn: Option<String>,
        ctx_area_fk100: Option<String>,
        ctx_area_nk100: Option<String>,
    ) -> Result<response::stock::order::balance::InquireBalance, Error> {
        let tr_id = match self.environment {
            Environment::Real => TrId::RealInquireBalance,
            Environment::Virtual => TrId::VirtualInquireBalance,
        };
        let param = request::stock::order::Query::InquireBalance::new(
            self.account.cano.clone(),
            self.account.acnt_prdt_cd.clone(),
            afhr_flpr_yn,
            inqr_dvsn,
            prcs_dvsn,
            ctx_area_fk100,
            ctx_area_nk100,
        );
        let url = format!(
            "{}/uapi/domestic-stock/v1/trading/inquire-balance",
            self.endpoint_url
        );
        let params = param.into_iter();
        let url = reqwest::Url::parse_with_params(&url, &params)?;
        let tr_id_str: String = tr_id.into();
        crate::wait(self.environment).await;
        let result = self
            .client
            .get(url)
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
            .header("tr_id", tr_id_str)
            .send()
            .await?
            .json::<response::stock::order::balance::InquireBalance>()
            .await?;
        crate::update_last_call();
        Ok(result)
    }

    /// 주식정정취소가능주문조회[v1_국내주식-004]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl)
    ///
    /// 정정 또는 취소가 가능한 미체결 주문 목록을 조회합니다.
    /// 실전계좌: 최대 50건, 모의계좌: 최대 20건 / 초과분은 `ctx_area_fk100`/`ctx_area_nk100`으로 연속조회
    pub async fn inquire_psbl_rvsecncl(
        &self,
        ctx_area_fk100: Option<String>,
        ctx_area_nk100: Option<String>,
        inqr_dvsn_1: Option<String>,
        inqr_dvsn_2: Option<String>,
    ) -> Result<response::stock::order::Body::InquirePsblRvsecncl, Error> {
        let tr_id = match self.environment {
            Environment::Real => TrId::RealInquirePsblRvsecncl,
            Environment::Virtual => TrId::VirtualInquirePsblRvsecncl,
        };
        let param = request::stock::order::Query::InquirePsblRvsecncl::new(
            self.account.cano.clone(),
            self.account.acnt_prdt_cd.clone(),
            ctx_area_fk100,
            ctx_area_nk100,
            inqr_dvsn_1,
            inqr_dvsn_2,
        );
        let url = format!(
            "{}/uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl",
            self.endpoint_url
        );
        let params = param.into_iter();
        let url = reqwest::Url::parse_with_params(&url, &params)?;
        let tr_id_str: String = tr_id.into();
        crate::wait(self.environment).await;
        let result = self
            .client
            .get(url)
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
            .header("tr_id", tr_id_str)
            .send()
            .await?
            .json::<response::stock::order::Body::InquirePsblRvsecncl>()
            .await?;
        crate::update_last_call();
        Ok(result)
    }

    // TODO: 매수가능조회[v1_국내주식-007]
    // [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/inquire-psbl-order)
}

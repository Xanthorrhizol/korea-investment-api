use crate::types::{request, response, Account, Environment, MarketCode, PeriodCode, TrId};
use crate::{auth, Error};

#[derive(Clone)]
pub struct Quote {
    client: reqwest::Client,
    endpoint_url: String,
    environment: Environment,
    auth: auth::Auth,
    account: Account,
}

impl Quote {
    /// 국내주식시세에 관한 API
    /// [국내주식시세](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock-quotations#L_07802512-4f49-4486-91b4-1050b6f5dc9d)
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

    /// 주식현재가 일자별[v1_국내주식-010]
    pub async fn daily_price(
        &self,
        market_code: MarketCode,
        shortcode: &str,
        period_code: PeriodCode,
        is_adjust_price: bool,
    ) -> Result<response::stock::quote::DailyPriceResponse, Error> {
        let tr_id = TrId::DailyPrice;
        let param = request::stock::quote::DailyPriceParameter::new(
            market_code,
            shortcode.to_string(),
            period_code,
            is_adjust_price,
        );
        let url = format!(
            "{}/uapi/domestic-stock/v1/quotations/inquire-daily-price",
            self.endpoint_url
        );
        let params = param.into_iter();
        let url = reqwest::Url::parse_with_params(&url, &params)?;
        Ok(self
            .create_request(tr_id, url)?
            .send()
            .await?
            .json::<response::stock::quote::DailyPriceResponse>()
            .await?)
    }

    /// 거래량순위[v1_국내주식-047]
    pub async fn volume_rank(
        &self,
        params: request::stock::quote::VolumeRankParameter,
    ) -> Result<response::stock::quote::VolumeRankResponse, Error> {
        let tr_id = TrId::VolumeRank;
        let url = format!(
            "{}/uapi/domestic-stock/v1/quotations/volume-rank",
            "https://openapi.koreainvestment.com:9443", // no VirtualMarket support
        );
        let url = reqwest::Url::parse_with_params(&url, &params.into_iter())?;
        Ok(self
            .create_request(tr_id, url)?
            .send()
            .await?
            .json::<response::stock::quote::VolumeRankResponse>()
            .await?)
    }

    /// 관심종목 그룹별 종목조회[국내주식-203]
    pub async fn group_item(
        &self,
        params: request::stock::quote::GroupItemParameter,
    ) -> Result<response::stock::quote::GroupItemResponse, Error> {
        let tr_id = TrId::InstockGroupItem;
        let url = format!(
            "{}/uapi/domestic-stock/v1/quotations/intstock-stocklist-by-group",
            "https://openapi.koreainvestment.com:9443", // no VirtualMarket support
        );
        let url = reqwest::Url::parse_with_params(&url, &params.into_iter())?;
        Ok(self
            .create_request(tr_id, url)?
            .send()
            .await?
            .json::<response::stock::quote::GroupItemResponse>()
            .await?)
    }

    /// 관심종목 그룹조회[국내주식-204]
    pub async fn group_list(
        &self,
        params: request::stock::quote::GroupListParameter,
    ) -> Result<response::stock::quote::GroupListResponse, Error> {
        let tr_id = TrId::InstockGrouplist;
        let url = format!(
            "{}/uapi/domestic-stock/v1/quotations/intstock-grouplist",
            "https://openapi.koreainvestment.com:9443", // no VirtualMarket support
        );
        let url = reqwest::Url::parse_with_params(&url, &params.into_iter())?;
        Ok(self
            .create_request(tr_id, url)?
            .send()
            .await?
            .json::<response::stock::quote::GroupListResponse>()
            .await?)
    }
    /// 주식기본조회[v1_국내주식-067]
    pub async fn basic_stock_info(
        &self,
        prdt_type_cd: &str,
        pdno: &str,
    ) -> Result<response::stock::quote::BasicStockInfoResponse, Error> {
        let tr_id = TrId::BasicStockInfo;
        let param = request::stock::quote::BasicStockInfoParameter::new(prdt_type_cd, pdno);
        let url = format!(
            "{}/uapi/domestic-stock/v1/quotations/search-stock-info",
            "https://openapi.koreainvestment.com:9443", // no VirtualMarket support
        );
        let params = param.into_iter();
        let url = reqwest::Url::parse_with_params(&url, &params)?;
        Ok(self
            .create_request(tr_id, url)?
            .send()
            .await?
            .json::<response::stock::quote::BasicStockInfoResponse>()
            .await?)
    }

    fn create_request(&self, tr_id: TrId, url: url::Url) -> Result<reqwest::RequestBuilder, Error> {
        Ok(self
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
            .header("tr_id", Into::<String>::into(tr_id))
            .header("custtype", "P"))
    }
}

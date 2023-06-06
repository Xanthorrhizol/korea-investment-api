mod auth;
mod data;
mod stock;
pub mod types;

/// 투자환경
/// 실전투자: Real
/// 모의투자: Virtual
#[derive(Clone)]
pub enum Environment {
    Real,
    Virtual,
}

/// 계좌
/// cano: CANO(계좌번호 체계(8-2)의 앞 8자리)
/// acnt_prdt_cd: ACNT_PRDT_CD(계좌번호 체계(8-2)의 뒤 2자리)
#[derive(Clone)]
pub struct Account {
    pub cano: String,
    pub acnt_prdt_cd: String,
}

#[derive(Clone)]
pub struct KoreaInvestmentApi<'a> {
    client: reqwest::Client,
    pub auth: auth::Auth,
    pub stock: stock::Korea,
    pub k_data: data::KoreaStockData<'a>,
    usehash: bool,
}

impl<'a> KoreaInvestmentApi<'a> {
    pub async fn new(
        acc: Environment,
        appkey: String,
        appsecret: String,
        account: Account,
        usehash: bool,
    ) -> Result<KoreaInvestmentApi<'a>, Error> {
        let client = reqwest::Client::new();
        let mut auth = auth::Auth::new(&client, acc.clone(), appkey, appsecret);
        auth.create_token().await?;
        auth.create_approval_key().await?;
        let stock =
            stock::Korea::new(&client, acc.clone(), auth.clone(), account.clone(), usehash)?;
        let k_data =
            data::KoreaStockData::new(acc.clone(), auth.clone(), account.clone(), usehash)?;
        Ok(Self {
            client,
            auth,
            stock,
            k_data,
            usehash,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // from lib
    #[error(transparent)]
    WebSocket(#[from] websocket::WebSocketError),
    #[error(transparent)]
    WebSocketParseError(#[from] websocket::url::ParseError),
    #[error(transparent)]
    WebSocketNativeTlsError(#[from] websocket::native_tls::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error(transparent)]
    JsonError(#[from] json::JsonError),
    #[error(transparent)]
    ChronoError(#[from] chrono::ParseError),

    // custom
    #[error("Auth init failed - None value in {0}")]
    AuthInitFailed(&'static str),
    #[error("Broken protocol - {0}: {1}")]
    BrokenProtocol(&'static str, String),
    #[error("The remote websocket server sent invalid data")]
    InvalidData,
}

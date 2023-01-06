mod auth;
mod stock;
mod types;

/// 투자환경
/// 실전투자: Real
/// 모의투자: Virtual
pub enum Environment {
    Real,
    Virtual,
}

/// 계좌
/// cano: CANO(계좌번호 체계(8-2)의 앞 8자리)
/// acnt_prdt_cd: ACNT_PRDT_CD(계좌번호 체계(8-2)의 뒤 2자리)
pub struct Account {
    pub cano: String,
    pub acnt_prdt_cd: String,
}

pub struct KoreaInvestmentApi {
    client: reqwest::Client,
    endpoint_url: String,
    auth: auth::Auth,
    stock: stock::Korea,
}

impl KoreaInvestmentApi {
    pub async fn new(
        acc: Environment,
        appkey: String,
        appsecret: String,
        account: Account,
    ) -> Result<Self, Error> {
        let endpoint_url = match acc {
            Environment::Real => "https://openapi.koreainvestment.com:9443",
            Environment::Virtual => "https://openapivts.koreainvestment.com:29443",
        }
        .to_string();
        let client = reqwest::Client::new();
        let mut auth = auth::Auth::new(&client, &endpoint_url, appkey, appsecret);
        auth.create_token().await?;
        auth.create_approval_key().await?;
        let stock = stock::Korea::new(&client, &endpoint_url, acc, &auth, account)?; // unwrap is safe here
        Ok(Self {
            client,
            endpoint_url,
            auth,
            stock,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // from lib
    #[error("Web socket error")]
    WebSocket(#[from] websocket::WebSocketError),
    #[error("Web socket parse error")]
    WebSocketParseError(#[from] websocket::url::ParseError),
    #[error("Web socket native_tls error")]
    WebSocketNativeTlsError(#[from] websocket::native_tls::Error),
    #[error("Reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    // custom
    #[error("Auth init failed - None value in {0}")]
    AuthInitFailed(&'static str),
}

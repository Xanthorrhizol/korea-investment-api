mod auth;
mod stock;
mod types;

/// 사용할 계좌
/// 실전투자: Real
/// 모의투자: Virtual
pub enum Account {
    Real,
    Virtual,
}

pub struct KoreaInvestmentApi {
    client: reqwest::Client,
    endpoint_url: String,
    auth: auth::Auth,
}

impl KoreaInvestmentApi {
    pub fn new(acc: Account, appkey: String, appsecret: String) -> Self {
        let endpoint_url = match acc {
            Account::Real => "https://openapi.koreainvestment.com:9443",
            Account::Virtual => "https://openapivts.koreainvestment.com:29443",
        }
        .to_string();
        let client = reqwest::Client::new();
        let auth = auth::Auth::new(&client, &endpoint_url, appkey, appsecret);
        Self {
            client,
            endpoint_url,
            auth,
        }
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
}

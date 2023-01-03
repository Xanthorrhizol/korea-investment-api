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
    stock: stock::Korea,
}

impl KoreaInvestmentApi {
    pub async fn new(acc: Account, appkey: String, appsecret: String) -> Result<Self, Error> {
        let endpoint_url = match acc {
            Account::Real => "https://openapi.koreainvestment.com:9443",
            Account::Virtual => "https://openapivts.koreainvestment.com:29443",
        }
        .to_string();
        let client = reqwest::Client::new();
        let mut auth = auth::Auth::new(&client, &endpoint_url, appkey, appsecret);
        auth.create_approval_key().await?;
        auth.create_hash().await?;
        let stock = stock::Korea::new(
            &endpoint_url,
            auth.get_approval_key().unwrap(),
            auth.get_hash().unwrap(),
        )?; // unwrap is safe here
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
}

mod auth;
mod stock;
pub mod types;
pub use types::{Account, Environment};

pub(crate) mod util;

#[macro_use]
extern crate log;

pub const BUF_SIZE: usize = 4096;

pub struct KoreaInvestmentApi {
    client: reqwest::Client,
    pub auth: auth::Auth,
    pub order: stock::order::Korea,
    pub quote: stock::quote::Quote,
    pub k_data: stock::data::KoreaStockData,
}

impl KoreaInvestmentApi {
    pub async fn new(
        acc: types::Environment,
        appkey: &str,
        appsecret: &str,
        account: types::Account,
        hts_id: &str,
        token: Option<String>,
        approval_key: Option<String>,
    ) -> Result<KoreaInvestmentApi, Error> {
        let client = reqwest::Client::new();
        let mut auth = auth::Auth::new(&client, acc.clone(), appkey, appsecret);
        info!(
            "Authorizing: acc={}, appkey={}, appsecret={}",
            &acc, &appkey, &appsecret,
        );
        if let Some(token) = token {
            auth.set_token(token);
        } else {
            auth.create_token().await?;
        }
        debug!("token: {:?}", auth.get_token());
        if let Some(approval_key) = approval_key {
            auth.set_approval_key(approval_key);
        } else {
            auth.create_approval_key().await?;
        }
        debug!("approval_key: {:?}", auth.get_approval_key());
        let order = stock::order::Korea::new(&client, acc.clone(), auth.clone(), account.clone())?;
        let quote = stock::quote::Quote::new(&client, acc.clone(), auth.clone(), account.clone())?;
        let k_data =
            stock::data::KoreaStockData::new(acc.clone(), auth.clone(), account.clone(), hts_id)?;
        info!("API Ready");
        Ok(Self {
            client,
            auth,
            order,
            quote,
            k_data,
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
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    // custom
    #[error("Auth init failed - None value in {0}")]
    AuthInitFailed(&'static str),
    #[error("Broken protocol - {0}: {1}")]
    BrokenProtocol(&'static str, String),
    #[error("The remote websocket server sent invalid data")]
    InvalidData,
    #[error("Wrong TrId: {0:?}. Expect {1}")]
    WrongTrId(crate::types::TrId, &'static str),
    #[error("AES cipher length error")]
    AesInvalidLength,
    #[error("AES decrypt error: {0}")]
    AesDecryptError(String),
}

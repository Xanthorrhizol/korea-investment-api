mod auth;
mod stock;
pub mod types;
pub(crate) mod util;

#[macro_use]
extern crate log;

pub const BUF_SIZE: usize = 4096;
pub const VIRTUAL_FREQ: i64 = 2;
pub const REAL_FREQ: i64 = 20;
lazy_static::lazy_static! {
    pub static ref LAST_CALL: std::sync::Mutex<types::Time> = std::sync::Mutex::new(types::Time::now());
}

pub(crate) async fn wait(env: types::Environment) {
    let freq = if env == types::Environment::Virtual {
        VIRTUAL_FREQ
    } else {
        REAL_FREQ
    };
    tokio::time::sleep(std::time::Duration::from_millis(
        (LAST_CALL.lock().unwrap().inner().timestamp_millis() + 1000 / freq
            - chrono::Utc::now().timestamp_millis())
        .max(0) as u64,
    ))
    .await;
}
pub(crate) fn update_last_call() {
    *LAST_CALL.lock().unwrap() = types::Time::now();
}

#[derive(Debug)]
pub struct KoreaInvestmentApi {
    pub auth: auth::Auth,
    pub real_auth: auth::Auth,
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
        real_appkey: Option<String>,
        real_appsecret: Option<String>,
        real_token: Option<String>,
        real_approval_key: Option<String>,
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

        let mut real_auth = if acc == types::Environment::Virtual {
            auth::Auth::new(
                &client,
                types::Environment::Real,
                &real_appkey.unwrap(),
                &real_appsecret.unwrap(),
            )
        } else {
            auth.clone()
        };
        if let Some(token) = real_token {
            real_auth.set_token(token);
        } else if real_auth.get_token().is_none() {
            real_auth.create_token().await?;
        }
        debug!("실전투자계좌(view용) token: {:?}", auth.get_token());
        if let Some(approval_key) = real_approval_key {
            real_auth.set_approval_key(approval_key);
        } else if real_auth.get_approval_key().is_none() {
            real_auth.create_approval_key().await?;
        }
        debug!("실전투자계좌(view용) approval_key: {:?}", auth.get_token());

        let order = stock::order::Korea::new(&client, acc.clone(), auth.clone(), account.clone())?;
        let quote = stock::quote::Quote::new(
            &client,
            acc.clone(),
            auth.clone(),
            account.clone(),
            real_auth.clone(),
        )?;
        let k_data =
            stock::data::KoreaStockData::new(acc.clone(), auth.clone(), account.clone(), hts_id)?;
        info!("API Ready");
        Ok(Self {
            auth,
            real_auth,
            order,
            quote,
            k_data,
        })
    }

    pub fn export_config(&self, config: &types::config::Config) -> Result<(), Error> {
        let mut config = config.clone();
        config.set_approval_key(self.auth.get_approval_key());
        config.set_token(self.auth.get_token());
        config.set_real_approval_key(self.real_auth.get_approval_key());
        config.set_real_token(self.real_auth.get_token());
        let toml = toml::to_string(&config)?;
        std::fs::write("config.toml", toml)?;

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // from lib
    #[error(transparent)]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
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
    #[error(transparent)]
    TomlSerializeError(#[from] toml::ser::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
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

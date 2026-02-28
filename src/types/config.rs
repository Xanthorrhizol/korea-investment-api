use crate::types::Environment;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default, Getters, Setters)]
pub struct Config {
    #[getset(get = "pub")]
    hts_id: String,
    #[getset(get = "pub")]
    cano: String,
    #[getset(get = "pub")]
    acnt_prdt_cd: String,
    #[getset(get = "pub")]
    app_key: String,
    #[getset(get = "pub")]
    app_secret: String,
    #[getset(get = "pub", set = "pub")]
    approval_key: Option<String>,
    #[getset(get = "pub", set = "pub")]
    token: Option<String>,
    #[getset(get = "pub")]
    environment: Environment,
    #[getset(get = "pub")]
    real_appkey: Option<String>,
    #[getset(get = "pub")]
    real_appsecret: Option<String>,
    #[getset(get = "pub", set = "pub")]
    real_approval_key: Option<String>,
    #[getset(get = "pub", set = "pub")]
    real_token: Option<String>,
}

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
}

use crate::types::Environment;
use getset::Getters;

#[derive(serde::Deserialize, Debug, Clone, Default, Getters)]
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
    #[getset(get = "pub")]
    environment: Environment,
}

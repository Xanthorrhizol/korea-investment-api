use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApprovalKeyCreationBody {
    grant_type: String,
    secretkey: String,
    appkey: String,
}
impl ApprovalKeyCreationBody {
    pub fn new(secretkey: String, appkey: String) -> Self {
        Self {
            grant_type: "client_credentials".to_string(),
            secretkey,
            appkey,
        }
    }
}

pub type TokenCreationBody = ApprovalKeyCreationBody;

#[derive(Debug, Serialize)]
pub struct TokenRevokeBody {
    appkey: String,
    appsecret: String,
    token: String,
}
impl TokenRevokeBody {
    pub fn new(appkey: String, appsecret: String, token: String) -> Self {
        Self {
            appkey,
            appsecret,
            token,
        }
    }
}

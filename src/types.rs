#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HashKeyRequestHeader {
    content_type: String,
    appkey: String,
    appsecret: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HashKeyResponse {
    JsonBody: String,
    HASH: String,
}

impl HashKeyResponse {
    pub fn get_hash(&self) -> String {
        self.HASH.clone()
    }
}

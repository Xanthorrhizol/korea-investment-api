#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HashKeyRequestHeader {
    content_type: String,
    appkey: String,
    appsecret: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApprovalKeyCreationResponse {
    approval_key: String,
}
impl ApprovalKeyCreationResponse {
    pub fn get_approval_key(&self) -> String {
        self.approval_key.clone()
    }
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenCreationResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
}

impl TokenCreationResponse {
    pub fn get_access_token(&self) -> String {
        self.access_token.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenRevokeResponse {
    pub code: u32,
    pub message: String,
}

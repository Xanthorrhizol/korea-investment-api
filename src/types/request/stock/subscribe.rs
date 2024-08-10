use crate::types::{CustomerType, TrId};
use getset::{Getters, Setters};
use serde::Serialize;

/// 국내주식 실시간체결가[실시간-003]
/// 국내주식 실시간호가[실시간-004]
#[derive(Debug, Clone, Getters, Setters, Serialize)]
pub struct SubscribeRequest {
    #[getset(get = "pub", set = "pub")]
    header: Header,
    #[getset(get = "pub", set = "pub")]
    body: Body,
}

#[repr(i32)]
#[derive(Debug, Clone, Serialize)]
pub enum TrType {
    #[serde(rename = "1")]
    Register = 1,
    #[serde(rename = "2")]
    Unregister = 2,
}

#[derive(Debug, Clone, Getters, Setters, Serialize)]
pub struct Header {
    #[getset(get = "pub", set = "pub")]
    appkey: String,
    #[getset(get = "pub", set = "pub")]
    appsecret: String,
    #[getset(get = "pub", set = "pub")]
    personalseckey: String,
    #[getset(get = "pub", set = "pub")]
    custtype: CustomerType,
    #[getset(get = "pub", set = "pub")]
    tr_type: TrType,
    #[getset(get = "pub", set = "pub")]
    #[serde(rename = "Content-Type")]
    content_type: String,
}

#[derive(Debug, Clone, Getters, Setters, Serialize)]
pub struct Body {
    #[getset(get = "pub", set = "pub")]
    input: BodyInner,
}

#[derive(Debug, Clone, Getters, Setters, Serialize)]
pub struct BodyInner {
    #[getset(get = "pub", set = "pub")]
    tr_id: TrId,
    #[getset(get = "pub", set = "pub")]
    tr_key: String,
}

impl SubscribeRequest {
    pub fn new(
        appkey: String,
        appsecret: String,
        personalseckey: String,
        custtype: CustomerType,
        tr_key: String,
        tr_id: TrId,
    ) -> Self {
        Self {
            header: Header {
                appkey,
                appsecret,
                personalseckey,
                custtype,
                tr_type: TrType::Register,
                content_type: "text/plain".to_string(),
            },
            body: Body {
                input: BodyInner { tr_id, tr_key },
            },
        }
    }

    pub fn get_json_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}

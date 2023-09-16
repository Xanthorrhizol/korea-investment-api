use crate::types::{CustomerType, TrId};
use serde::Serialize;

/// 국내주식 실시간체결가[실시간-003]
/// 국내주식 실시간호가[실시간-004]
#[derive(Serialize)]
pub struct SubscribeRequest {
    header: Header,
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

#[derive(Debug, Serialize)]
pub struct Header {
    appkey: String,
    appsecret: String,
    personalseckey: String,
    custtype: CustomerType,
    tr_type: TrType,
    #[serde(rename = "Content-Type")]
    content_type: String,
}

#[derive(Clone, Serialize)]
pub struct Body {
    input: BodyInner,
}

#[derive(Clone, Serialize)]
pub struct BodyInner {
    tr_id: TrId,
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

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn get_json_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}

impl Header {
    pub fn appkey(&self) -> &str {
        &self.appkey
    }

    pub fn appsecret(&self) -> &str {
        &self.appsecret
    }

    pub fn personalseckey(&self) -> &str {
        &self.personalseckey
    }

    pub fn custtype(&self) -> &CustomerType {
        &self.custtype
    }

    pub fn tr_type(&self) -> &TrType {
        &self.tr_type
    }
}

impl Body {
    pub fn tr_id(&self) -> &TrId {
        &self.input.tr_id
    }

    pub fn tr_key(&self) -> &str {
        &self.input.tr_key
    }
}

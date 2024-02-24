use crate::types::{CustomerType, TrId};
use getset::{Getters, Setters};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Getters, Setters)]
pub struct SubscribeResponse {
    #[getset(get = "pub", set = "pub")]
    success: bool,
    #[getset(get = "pub", set = "pub")]
    msg: String,
    #[getset(get = "pub", set = "pub")]
    iv: Option<String>,
    #[getset(get = "pub", set = "pub")]
    key: Option<String>,
}

impl SubscribeResponse {
    pub fn new(success: bool, msg: String, iv: Option<String>, key: Option<String>) -> Self {
        Self {
            success,
            msg,
            iv,
            key,
        }
    }
}

/// 국내주식 실시간체결가[실시간-003]
/// 국내주식 실시간호가[실시간-004]
pub struct Subscribe {
    header: Header,
    body: Body,
}

#[derive(Debug, Clone)]
pub enum TrType {
    Register,
    Unregister,
}

impl Into<String> for TrType {
    fn into(self) -> String {
        match self {
            Self::Register => "1",
            Self::Unregister => "2",
        }
        .to_string()
    }
}

#[derive(Debug, Getters, Setters)]
pub struct Header {
    #[getset(get = "pub", set = "pub")]
    app_key: String,
    #[getset(get = "pub", set = "pub")]
    appsecret: String,
    #[getset(get = "pub", set = "pub")]
    personalseckey: String,
    #[getset(get = "pub", set = "pub")]
    custtype: CustomerType,
    #[getset(get = "pub", set = "pub")]
    tr_type: TrType,
}

#[derive(Clone, Getters, Setters)]
pub struct Body {
    #[getset(get = "pub", set = "pub")]
    tr_id: TrId,
    #[getset(get = "pub", set = "pub")]
    tr_key: String,
}

impl Subscribe {
    pub fn new(
        app_key: String,
        appsecret: String,
        personalseckey: String,
        custtype: CustomerType,
        tr_key: String,
        tr_id: TrId,
    ) -> Self {
        Self {
            header: Header {
                app_key,
                appsecret,
                personalseckey,
                custtype,
                tr_type: TrType::Register,
            },
            body: Body { tr_id, tr_key },
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn get_json_string(&self) -> String {
        format!(
            "{{\"header\":{{\"appkey\":\"{}\",\"appsecret\":\"{}\",\"personalseckey\":\"{}\",\"custtype\":\"{}\",\"tr_type\":\"{}\",\"content-type\":\"text/plain\"}},\"body\":{{\"input\": {{\"tr_id\":\"{}\",\"tr_key\":\"{}\"}}}}}}",
            self.header().app_key(),
            self.header().appsecret(),
            self.header().personalseckey(),
            Into::<String>::into(self.header().custtype().to_owned()),
            Into::<String>::into(self.header().tr_type().to_owned()),
            Into::<String>::into(self.body().tr_id().to_owned()),
            self.body().tr_key(),
        )
    }
}

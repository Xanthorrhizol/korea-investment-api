use crate::types::{CustomerType, TrId};

#[derive(Debug, Clone)]
pub struct SubscribeResult {
    success: bool,
    msg: String,
    iv: Option<String>,
    key: Option<String>,
}

impl SubscribeResult {
    pub fn new(success: bool, msg: String, iv: Option<String>, key: Option<String>) -> Self {
        Self {
            success,
            msg,
            iv,
            key,
        }
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn iv(&self) -> &Option<String> {
        &self.iv
    }

    pub fn key(&self) -> &Option<String> {
        &self.key
    }

    pub fn set_success(&mut self, success: bool) {
        self.success = success;
    }

    pub fn set_msg(&mut self, msg: String) {
        self.msg = msg;
    }

    pub fn set_iv(&mut self, iv: Option<String>) {
        self.iv = iv;
    }

    pub fn set_key(&mut self, key: Option<String>) {
        self.key = key;
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

#[derive(Debug)]
pub struct Header {
    app_key: String,
    appsecret: String,
    personalseckey: String,
    custtype: CustomerType,
    tr_type: TrType,
}

#[derive(Clone)]
pub struct Body {
    tr_id: TrId,
    tr_key: String,
}

impl Subscribe {
    pub fn new(
        app_key: String,
        appsecret: String,
        personalseckey: String,
        custtype: CustomerType,
        isin: String,
    ) -> Self {
        Self {
            header: Header {
                app_key,
                appsecret,
                personalseckey,
                custtype,
                tr_type: TrType::Register,
            },
            body: Body {
                tr_id: TrId::RealtimeExec,
                tr_key: isin,
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

impl Header {
    pub fn app_key(&self) -> &str {
        &self.app_key
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
        &self.tr_id
    }

    pub fn tr_key(&self) -> &str {
        &self.tr_key
    }
}

#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderDivision {
    Limit,
    Market,
    // TODO: add other types
}
impl Into<String> for OrderDivision {
    fn into(self) -> String {
        match self {
            OrderDivision::Limit => "00".to_string(),
            OrderDivision::Market => "01".to_string(),
            // TODO: add other types
        }
    }
}
impl From<String> for OrderDivision {
    fn from(s: String) -> Self {
        match s.as_str() {
            "00" => OrderDivision::Limit,
            "01" => OrderDivision::Market,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    inner: u32,
}
impl Quantity {
    fn from(quantity: u32) -> Self {
        Self { inner: quantity }
    }
}
impl Into<String> for Quantity {
    fn into(self) -> String {
        format!("{}", self.inner)
    }
}
impl From<String> for Quantity {
    fn from(s: String) -> Self {
        Self {
            inner: s.trim().parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    inner: u32,
}
impl Price {
    fn from(price: u32) -> Self {
        Self { inner: price }
    }
}
impl Into<String> for Price {
    fn into(self) -> String {
        format!("{}", self.inner)
    }
}
impl From<String> for Price {
    fn from(s: String) -> Self {
        Self {
            inner: s.trim().parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrId {
    RealStockCashBidOrder,
    RealStockCashAskOrder,
    VirtualStockCashBidOrder,
    VirtualStockCashAskOrder,
    // TODO: other
}
impl Into<String> for TrId {
    fn into(self) -> String {
        match self {
            TrId::RealStockCashBidOrder => "TTTC0802U",
            TrId::RealStockCashAskOrder => "TTTC0801U",
            TrId::VirtualStockCashBidOrder => "VTTC0802U",
            TrId::VirtualStockCashAskOrder => "VTTC0801U",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CustomerType {
    Business,
    Personal,
}
impl Into<String> for CustomerType {
    fn into(self) -> String {
        match self {
            CustomerType::Business => "B",
            CustomerType::Personal => "P",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestHeader {
    authorization: String,
    appkey: String,
    appsecret: String,
    // personalseckey: String // TODO: 법인용
    tr_id: TrId,
    custtype: CustomerType,
}

impl RequestHeader {
    fn new(token: String, appkey: String, appsecret: String, tr_id: TrId) -> Self {
        Self {
            authorization: token,
            appkey,
            appsecret,
            tr_id,
            custtype: CustomerType::Personal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestBody {
    CANO: String,            // 종합계좌번호(계좌번호 체계(8-2)의 앞 8자리)
    ACNT_PRDT_CD: String,    // 계좌상품코드(계좌번호 체계(8-2)의 뒤 2자리)
    PDNO: String,            // 종목코드(6자리)
    ORD_DVSN: OrderDivision, // 주문구분
    ORD_QTY: Quantity,       // 주문수량(주문주식수)
    ORD_UNPR: Price,         // 주문단가(1주당 가격; 시장가는 0으로)
}

impl RequestBody {
    pub fn new(
        CANO: String,
        ACNT_PRDT_CD: String,
        PDNO: String,
        ORD_DVSN: OrderDivision,
        ORD_QTY: Quantity,
        ORD_UNPR: Price,
    ) -> Self {
        Self {
            CANO,
            ACNT_PRDT_CD,
            PDNO,
            ORD_DVSN,
            ORD_QTY,
            ORD_UNPR,
        }
    }
    pub fn get_json_string(self) -> String {
        format!(
            "{{\"CANO\":\"{}\",\"ACNT_PRDT_CD\":\"{}\",\"PDNO\":\"{}\",\"ORD_DVSN\":\"{}\",\"ORD_QTY\":\"{}\", \"ORD_UNPR\":\"{}\"}}", self.CANO, self.ACNT_PRDT_CD, self.PDNO, Into::<String>::into(self.ORD_DVSN), Into::<String>::into(self.ORD_QTY), Into::<String>::into(self.ORD_UNPR))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseBody {
    tr_cd: String,  // 0: 성공, 0 이외의 값: 실패
    msg_cd: String, // 응답코드
    msg: String,    // 응답메시지
    output: Output,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
    KRX_FWDG_ORD_ORGNO: String, // 주문시 한국투자증권 시스템에서 지정된 영업점코드
    ODNO: String,               // 주문번호(주문시 한국투자증권 시스템에서 채번된 주문번호)
    ORD_TMD: String,            // 주문시각(시분초HHMMSS)
}

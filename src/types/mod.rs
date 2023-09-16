mod crypto;
mod exec;
mod my_exec;
mod ordb;
pub mod request;
pub mod response;
mod time;

pub(crate) use crypto::Aes256CbcDec;
pub use exec::Exec;
pub use my_exec::MyExec;
pub use ordb::Ordb;
pub use request::subscribe::SubscribeRequest;
pub use response::subscribe::SubscribeResponse;
use serde::{Deserialize, Serialize};
pub use time::Time;

pub fn parse_bool(s: &str) -> bool {
    match s.to_ascii_uppercase().as_str() {
        "1" | "TRUE" | "T" | "Y" => true,
        "0" | "FALSE" | "F" | "N" => false,
        _ => false,
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    tr_id: TrId,
    datetime: Time,
}
impl Header {
    pub fn tr_id(&self) -> &TrId {
        &self.tr_id
    }

    pub fn datetime(&self) -> &Time {
        &self.datetime
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderClass {
    Limit,                // 지정가
    Market,               // 시장가
    ConditionalLimit,     // 조건부지정가
    Best,                 // 최유리지정가
    First,                // 최우선지정가
    PreMarket,            // 장전시간외
    PostMarket,           // 장후시간외
    OutMarketSinglePrice, // 시간외단일가
    MyStock,              // 자기주식
    MyStockSOption,       // 자기주식S-Option
    MyStockMoneyTrust,    // 자시주식금전신탁
    IOCLimit,             // IOC지정가
    FOKLimit,             // FOK지정가
    IOCMarket,            // IOC시장가
    FOKMarket,            // FOK시장가
    IOCBest,              // IOC최유리
    FOKBest,              // FOK최유리
}
impl Into<String> for OrderClass {
    fn into(self) -> String {
        match self {
            Self::Limit => "00".to_string(),
            Self::Market => "01".to_string(),
            Self::ConditionalLimit => "02".to_string(),
            Self::Best => "03".to_string(),
            Self::First => "04".to_string(),
            Self::PreMarket => "05".to_string(),
            Self::PostMarket => "06".to_string(),
            Self::OutMarketSinglePrice => "07".to_string(),
            Self::MyStock => "08".to_string(),
            Self::MyStockSOption => "09".to_string(),
            Self::MyStockMoneyTrust => "10".to_string(),
            Self::IOCLimit => "11".to_string(),
            Self::FOKLimit => "12".to_string(),
            Self::IOCMarket => "13".to_string(),
            Self::FOKMarket => "14".to_string(),
            Self::IOCBest => "15".to_string(),
            Self::FOKBest => "16".to_string(),
        }
    }
}
impl From<&str> for OrderClass {
    fn from(s: &str) -> Self {
        match s {
            "00" => OrderClass::Limit,
            "01" => OrderClass::Market,
            "02" => OrderClass::ConditionalLimit,
            "03" => OrderClass::Best,
            "04" => OrderClass::First,
            "05" => OrderClass::PreMarket,
            "06" => OrderClass::PostMarket,
            "07" => OrderClass::OutMarketSinglePrice,
            "08" => OrderClass::MyStock,
            "09" => OrderClass::MyStockSOption,
            "10" => OrderClass::MyStockMoneyTrust,
            "11" => OrderClass::IOCLimit,
            "12" => OrderClass::FOKLimit,
            "13" => OrderClass::IOCMarket,
            "14" => OrderClass::FOKMarket,
            "15" => OrderClass::IOCBest,
            "16" => OrderClass::FOKBest,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CorrectionClass {
    None,
    Correction,
    Cancel,
}
impl Into<String> for CorrectionClass {
    fn into(self) -> String {
        match self {
            Self::None => "0",
            Self::Correction => "01",
            Self::Cancel => "02",
        }
        .to_string()
    }
}
impl From<&str> for CorrectionClass {
    fn from(s: &str) -> CorrectionClass {
        match s {
            "0" => CorrectionClass::None,
            "01" => CorrectionClass::Correction,
            "02" => CorrectionClass::Cancel,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Bid, // buy
    Ask, // sell
}

impl From<&str> for Direction {
    fn from(direction: &str) -> Self {
        match direction {
            "01" => Self::Ask,
            "02" => Self::Bid,
            _ => unreachable!(),
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
impl From<&str> for Quantity {
    fn from(s: &str) -> Self {
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
impl From<&str> for Price {
    fn from(s: &str) -> Self {
        Self {
            inner: s.trim().parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrId {
    // Order
    #[serde(rename = "TTTC0802U")]
    RealStockCashBidOrder,
    #[serde(rename = "TTTC0801U")]
    RealStockCashAskOrder,
    #[serde(rename = "VTTC0802U")]
    VirtualStockCashBidOrder,
    #[serde(rename = "VTTC0801U")]
    VirtualStockCashAskOrder,
    // Correction
    #[serde(rename = "TTTC0803U")]
    RealStockCorrection,
    #[serde(rename = "VTTC0803U")]
    VirtualStockCorrection,
    // Market data
    #[serde(rename = "H0STCNT0")]
    RealtimeExec,
    #[serde(rename = "H0STASP0")]
    RealtimeOrdb,
    #[serde(rename = "H0STCNI0")]
    RealRealtimeMyExec,
    #[serde(rename = "H0STCNI9")]
    VirtualRealtimeMyExec,
    // PingPong
    #[serde(rename = "PINGPONG")]
    PingPong,
}
impl Into<String> for TrId {
    fn into(self) -> String {
        match self {
            // Order
            TrId::RealStockCashBidOrder => "TTTC0802U",
            TrId::RealStockCashAskOrder => "TTTC0801U",
            TrId::VirtualStockCashBidOrder => "VTTC0802U",
            TrId::VirtualStockCashAskOrder => "VTTC0801U",
            // Correction
            TrId::RealStockCorrection => "TTTC0803U",
            TrId::VirtualStockCorrection => "VTTC0803U",
            // Market data
            TrId::RealtimeExec => "H0STCNT0",
            TrId::RealtimeOrdb => "H0STASP0",
            TrId::RealRealtimeMyExec => "H0STCNI0",
            TrId::VirtualRealtimeMyExec => "H0STCNI9",
            // PingPong
            TrId::PingPong => "PINGPONG",
        }
        .to_string()
    }
}

impl From<&str> for TrId {
    fn from(s: &str) -> Self {
        match s {
            // Order
            "TTTC0802U" => TrId::RealStockCashBidOrder,
            "TTTC0801U" => TrId::RealStockCashAskOrder,
            "VTTC0802U" => TrId::VirtualStockCashBidOrder,
            "VTTC0801U" => TrId::VirtualStockCashAskOrder,
            // Correction
            "TTTC0803U" => TrId::RealStockCorrection,
            "VTTC0803U" => TrId::VirtualStockCorrection,
            // Market data
            "H0STCNT0" => TrId::RealtimeExec,
            "H0STASP0" => TrId::RealtimeOrdb,
            "H0STCNI0" => TrId::RealRealtimeMyExec,
            "H0STCNI9" => TrId::VirtualRealtimeMyExec,
            // PingPong
            "PINGPONG" => TrId::PingPong,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CustomerType {
    #[serde(rename = "B")]
    Business,
    #[serde(rename = "P")]
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

/// 체결구분
#[derive(Debug, Clone)]
pub enum ExecClass {
    Bid,       // 매수(1)
    PreMarket, // 장전(3)
    Ask,       // 매도(5)
}

impl From<&str> for ExecClass {
    fn from(s: &str) -> Self {
        match s {
            "1" => Self::Bid,
            "3" => Self::PreMarket,
            "5" => Self::Ask,
            _ => unreachable!(),
        }
    }
}

/// 대비구분
#[derive(Clone, Debug)]
pub enum VsPriceSign {
    UpperLimit, // 상한(1)
    Increase,   // 상승(2)
    Steady,     // 보합(3)
    Decrease,   // 하락(4)
    LowerLimit, // 하한(5)
}

impl From<&str> for VsPriceSign {
    fn from(s: &str) -> Self {
        match s {
            "1" => Self::UpperLimit,
            "2" => Self::Increase,
            "3" => Self::Steady,
            "4" => Self::Decrease,
            "5" => Self::LowerLimit,
            _ => unreachable!(),
        }
    }
}

/// 시간 구분 코드
#[derive(Clone, Debug)]
pub enum TimeClassCode {
    InMarket,                    // 장중(0)
    PostMarketPrediction,        // 장후예상(A)
    PreMarketPrediction,         // 장전예상(B)
    PostNinePmPredictionOrVi,    // 9시이후의 예상가, VI발동(C)
    OutMarketSinglePricePredict, // 시간외 단일가 예상(D)
}

impl From<&str> for TimeClassCode {
    fn from(s: &str) -> Self {
        match s {
            "0" => Self::InMarket,
            "A" => Self::PostMarketPrediction,
            "B" => Self::PreMarketPrediction,
            "C" => Self::PostNinePmPredictionOrVi,
            "D" => Self::OutMarketSinglePricePredict,
            _ => unreachable!(),
        }
    }
}

/// 매매구분코드
// TODO: check and find what it is
#[derive(Debug, Clone)]
pub enum DealClassCode {
    IDK(String),
}
impl From<&str> for DealClassCode {
    fn from(s: &str) -> Self {
        Self::IDK(s.to_string())
    }
}

/// 임의종료구분코드
// TODO: check
#[derive(Debug, Clone)]
pub enum MarketTerminationClassCode {
    Terminated,
}

impl From<&str> for MarketTerminationClassCode {
    fn from(_s: &str) -> Self {
        Self::Terminated
    }
}

/// 장운영 구분 코드
#[derive(Clone, Debug)]
pub struct MarketOperationClassCode(When, What);

impl MarketOperationClassCode {
    pub fn when(&self) -> &When {
        &self.0
    }

    pub fn what(&self) -> &What {
        &self.1
    }
}

impl From<&str> for MarketOperationClassCode {
    fn from(s: &str) -> Self {
        let first = match &s[0..1] {
            "1" => When::PreMarket,
            "2" => When::Market,
            "3" => When::PostMarket,
            "4" => When::OutMarketSinglePrice,
            "7" => When::NormalBuyIn,
            "8" => When::TodayBuyIn,
            _ => unreachable!(),
        };
        let second = match &s[1..2] {
            "0" => What::Normal,
            "1" => What::EndPrice,
            "2" => What::Massive,
            "3" => What::Basket,
            "7" => What::Clearance,
            "7" => What::BuyIn,
            _ => unreachable!(),
        };
        Self(first, second)
    }
}

/// 장운영 구분 코드(1st bit) - 주문 시점
#[derive(Clone, Debug)]
pub enum When {
    PreMarket,            // 장개시전(1)
    Market,               // 장중(2)
    PostMarket,           // 장종료후(3)
    OutMarketSinglePrice, // 시간외단일가(4)
    NormalBuyIn,          // 일반 Buy-in(7)
    TodayBuyIn,           // 당일Buy-in(8)
}

/// 장운영 구분 코드(2nd bit) - 주문 대상
#[derive(Clone, Debug)]
pub enum What {
    Normal,    // 보통(0)
    EndPrice,  // 종가(1)
    Massive,   // 대량(2)
    Basket,    // 바스켓(3)
    Clearance, // 정리매매(7)
    BuyIn,     // Buy-in(8)
}

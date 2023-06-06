mod exec;
pub mod request;
pub mod response;
mod subscribe;
mod time;

pub use exec::Exec;
use serde::{Deserialize, Serialize};
pub use subscribe::Subscribe;
pub use time::Time;

#[derive(Debug, Clone)]
pub struct Header {
    tr_id: TrId,
    datetime: Time,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderDivision {
    Limit,
    Market,
    // TODO: add other types
}
impl Into<String> for OrderDivision {
    fn into(self) -> String {
        match self {
            Self::Limit => "00".to_string(),
            Self::Market => "01".to_string(),
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
pub enum CorrectionDivision {
    Correction,
    Cancel,
}
impl Into<String> for CorrectionDivision {
    fn into(self) -> String {
        match self {
            Self::Correction => "01",
            Self::Cancel => "02",
        }
        .to_string()
    }
}
impl From<String> for CorrectionDivision {
    fn from(s: String) -> CorrectionDivision {
        match s.as_str() {
            "01" => CorrectionDivision::Correction,
            "02" => CorrectionDivision::Cancel,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Bid, // buy
    Ask, // sell
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
    // Order
    RealStockCashBidOrder,
    RealStockCashAskOrder,
    VirtualStockCashBidOrder,
    VirtualStockCashAskOrder,
    // Correction
    RealStockCorrection,
    VirtualStockCorrection,
    // Market data
    RealtimeExec,
    RealtimeOrdb,
    // PingPong
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
            // PingPong
            TrId::PingPong => "PINGPONG",
        }
        .to_string()
    }
}

impl From<String> for TrId {
    fn from(s: String) -> Self {
        match s.as_str() {
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
            // PingPong
            "PINGPONG" => TrId::PingPong,
            _ => todo!(),
        }
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

/// 체결구분
#[derive(Debug, Clone)]
pub enum ExecClass {
    All,     // 전체(00)
    Partial, // 체결(01)
    Nothing, // 미체결(02)
}

impl From<String> for ExecClass {
    fn from(s: String) -> Self {
        match s.as_str() {
            "00" => Self::All,
            "01" => Self::Partial,
            "02" => Self::Nothing,
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

impl From<String> for VsPriceSign {
    fn from(s: String) -> Self {
        match s.as_str() {
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

impl From<String> for TimeClassCode {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => Self::InMarket,
            "A" => Self::PostMarketPrediction,
            "B" => Self::PreMarketPrediction,
            "C" => Self::PostNinePmPredictionOrVi,
            "D" => Self::OutMarketSinglePricePredict,
            _ => unreachable!(),
        }
    }
}

/// 임의종료구분코드
// TODO: check
#[derive(Debug, Clone)]
pub enum MarketTerminationClassCode {
    Terminated,
}

impl From<String> for MarketTerminationClassCode {
    fn from(s: String) -> Self {
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

impl From<String> for MarketOperationClassCode {
    fn from(s: String) -> Self {
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

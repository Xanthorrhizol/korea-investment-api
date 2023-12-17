pub mod auth;
mod crypto;
pub mod stock;
mod time;

pub(crate) use crypto::Aes256CbcDec;
use serde::{Deserialize, Serialize};
pub use time::Time;

pub fn parse_bool(s: &str) -> bool {
    match s.to_ascii_uppercase().as_str() {
        "1" | "TRUE" | "T" | "Y" => true,
        "0" | "FALSE" | "F" | "N" => false,
        _ => false,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrId {
    // === Stock === //
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
    // Quote
    #[serde(rename = "FHKST01010400")]
    DailyStockPrice,
    // Market data
    #[serde(rename = "H0STCNT0")]
    RealtimeExec,
    #[serde(rename = "H0STASP0")]
    RealtimeOrdb,
    #[serde(rename = "H0STCNI0")]
    RealRealtimeMyExec,
    #[serde(rename = "H0STCNI9")]
    VirtualRealtimeMyExec,
    // === FutureOption === //
    // Order
    #[serde(rename = "TTTO1101U")]
    RealDaytimeFutureOptionOrder,
    #[serde(rename = "JTCE1001U")]
    RealNightFutureOptionOrder,
    #[serde(rename = "VTTO1101U")]
    VirtualDaytimeFutureOptionOrder,
    #[serde(rename = "VTCE1001U")]
    VirtualNightFutureOptionOrder,
    // === Connection === //
    // PingPong
    #[serde(rename = "PINGPONG")]
    PingPong,
}
impl Into<String> for TrId {
    fn into(self) -> String {
        match self {
            // === Stock === //
            // Order
            TrId::RealStockCashBidOrder => "TTTC0802U",
            TrId::RealStockCashAskOrder => "TTTC0801U",
            TrId::VirtualStockCashBidOrder => "VTTC0802U",
            TrId::VirtualStockCashAskOrder => "VTTC0801U",
            // Correction
            TrId::RealStockCorrection => "TTTC0803U",
            TrId::VirtualStockCorrection => "VTTC0803U",
            // Quote
            TrId::DailyStockPrice => "FHKST01010400",
            // Market data
            TrId::RealtimeExec => "H0STCNT0",
            TrId::RealtimeOrdb => "H0STASP0",
            TrId::RealRealtimeMyExec => "H0STCNI0",
            TrId::VirtualRealtimeMyExec => "H0STCNI9",
            // === FutureOption === //
            // Order
            TrId::RealDaytimeFutureOptionOrder => "TTTO1101U",
            TrId::RealNightFutureOptionOrder => "JTCE1001U",
            TrId::VirtualDaytimeFutureOptionOrder => "VTTO1101U",
            TrId::VirtualNightFutureOptionOrder => "VTCE1001U",
            // === Connection === //
            // PingPong
            TrId::PingPong => "PINGPONG",
        }
        .to_string()
    }
}
impl From<&str> for TrId {
    fn from(s: &str) -> Self {
        match s {
            // === Stock === //
            // Order
            "TTTC0802U" => TrId::RealStockCashBidOrder,
            "TTTC0801U" => TrId::RealStockCashAskOrder,
            "VTTC0802U" => TrId::VirtualStockCashBidOrder,
            "VTTC0801U" => TrId::VirtualStockCashAskOrder,
            // Correction
            "TTTC0803U" => TrId::RealStockCorrection,
            "VTTC0803U" => TrId::VirtualStockCorrection,
            // Quote
            "FHKST01010400" => TrId::DailyStockPrice,
            // Market data
            "H0STCNT0" => TrId::RealtimeExec,
            "H0STASP0" => TrId::RealtimeOrdb,
            "H0STCNI0" => TrId::RealRealtimeMyExec,
            "H0STCNI9" => TrId::VirtualRealtimeMyExec,
            // === FutureOption === //
            // Order
            "TTTO1101U" => TrId::RealDaytimeFutureOptionOrder,
            "JTCE1001U" => TrId::RealNightFutureOptionOrder,
            "VTTO1101U" => TrId::VirtualDaytimeFutureOptionOrder,
            "VTCE1001U" => TrId::VirtualNightFutureOptionOrder,
            // === Connection === //
            // PingPong
            "PINGPONG" => TrId::PingPong,
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

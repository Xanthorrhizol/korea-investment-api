pub mod request;
pub mod response;
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

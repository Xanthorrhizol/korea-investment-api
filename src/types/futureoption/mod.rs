pub mod request;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderClass {
    #[serde(rename = "01")]
    Limit, // 지정가
    #[serde(rename = "02")]
    Market, // 시장가
    #[serde(rename = "03")]
    ConditionalLimit, // 조건부지정가
    #[serde(rename = "04")]
    Best, // 최유리지정가
    #[serde(rename = "10")]
    IOCLimit, // IOC지정가
    #[serde(rename = "11")]
    FOKLimit, // FOK지정가
    #[serde(rename = "12")]
    IOCMarket, // IOC시장가
    #[serde(rename = "13")]
    FOKMarket, // FOK시장가
    #[serde(rename = "14")]
    IOCBest, // IOC최유리
    #[serde(rename = "15")]
    FOKBest, // FOK최유리
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderTypeClass {
    #[serde(rename = "01")]
    Limit, // 지정가
    #[serde(rename = "02")]
    Market, // 시장가
    #[serde(rename = "03")]
    ConditionalLimit, // 조건부지정가
    #[serde(rename = "04")]
    Best, // 최유리지정가
}
impl From<OrderClass> for OrderTypeClass {
    fn from(oc: OrderClass) -> Self {
        Self::from(oc)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KrxOrderClass {
    #[serde(rename = "0")]
    None,
    #[serde(rename = "3")]
    IOC,
    #[serde(rename = "4")]
    FOK,
}
impl From<OrderClass> for KrxOrderClass {
    fn from(oc: OrderClass) -> Self {
        match oc {
            OrderClass::IOCLimit | OrderClass::IOCMarket | OrderClass::IOCBest => Self::IOC,
            OrderClass::FOKLimit | OrderClass::FOKMarket | OrderClass::FOKBest => Self::FOK,
            _ => Self::None,
        }
    }
}

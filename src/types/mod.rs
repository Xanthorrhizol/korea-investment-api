#[cfg(feature = "example")]
pub mod config;
mod crypto;
pub mod request;
pub mod response;
pub mod stream;
mod time;

pub(crate) use crypto::Aes256CbcDec;
use serde::{Deserialize, Serialize};
use serde_with::SerializeDisplay;
pub use time::Time;

pub fn parse_bool(s: &str) -> bool {
    match s.to_ascii_uppercase().as_str() {
        "1" | "TRUE" | "T" | "Y" => true,
        "0" | "FALSE" | "F" | "N" => false,
        _ => false,
    }
}

/// 투자환경
/// 실전투자: Real
/// 모의투자: Virtual
#[derive(Clone, Debug, Default, serde_with::DeserializeFromStr, serde_with::SerializeDisplay)]
pub enum Environment {
    Real,
    #[default]
    Virtual,
}

impl std::str::FromStr for Environment {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, crate::Error> {
        Ok(match s.to_lowercase().as_str() {
            "real" => Self::Real,
            "virtual" => Self::Virtual,
            _ => Self::default(),
        })
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            Self::Real => "Real",
            Self::Virtual => "Virtual",
        })
    }
}

/// 계좌
/// cano: CANO(계좌번호 체계(8-2)의 앞 8자리)
/// acnt_prdt_cd: ACNT_PRDT_CD(계좌번호 체계(8-2)의 뒤 2자리)
#[derive(Clone)]
pub struct Account {
    pub cano: String,
    pub acnt_prdt_cd: String,
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
    pub fn from(quantity: u32) -> Self {
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Price {
    inner: u32,
}
impl Price {
    pub fn from(price: u32) -> Self {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
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
    // Quote
    #[serde(rename = "FHKST01010400")]
    DailyPrice,
    #[serde(rename = "FHPST01710000")]
    VolumeRank,
    #[serde(rename = "HHKCM113004C7")]
    InstockGrouplist,
    #[serde(rename = "HHKCM113004C6")]
    InstockGroupItem,
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
            // Quote
            TrId::DailyPrice => "FHKST01010400",
            TrId::VolumeRank => "FHPST01710000",
            TrId::InstockGrouplist => "HHKCM113004C7",
            TrId::InstockGroupItem => "HHKCM113004C6",
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
            // Quote
            "FHKST01010400" => TrId::DailyPrice,
            "FHPST01710000" => TrId::VolumeRank,
            "HHKCM113004C7" => TrId::InstockGrouplist,
            "HHKCM113004C6" => TrId::InstockGroupItem,
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
#[derive(Clone, Debug, Deserialize)]
pub enum VsPriceSign {
    #[serde(rename = "1")]
    UpperLimit, // 상한(1)
    #[serde(rename = "2")]
    Increase, // 상승(2)
    #[serde(rename = "3")]
    Steady, // 보합(3)
    #[serde(rename = "4")]
    Decrease, // 하락(4)
    #[serde(rename = "5")]
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

#[derive(Clone, Debug, Deserialize, SerializeDisplay)]
pub enum MarketCode {
    #[serde(rename = "J")]
    Stock,
    #[serde(rename = "ETF")]
    Etf,
}
impl std::fmt::Display for MarketCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Stock => "J",
            Self::Etf => "ETF",
        })
    }
}

#[derive(Clone, Debug, Deserialize, SerializeDisplay)]
pub enum PeriodCode {
    #[serde(rename = "D")]
    ThirtyDays,
    #[serde(rename = "W")]
    ThirtyWeeks,
    #[serde(rename = "M")]
    ThirtyMonths,
}
impl std::fmt::Display for PeriodCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::ThirtyDays => "D",
            Self::ThirtyWeeks => "W",
            Self::ThirtyMonths => "M",
        })
    }
}

#[derive(Clone, Debug, Deserialize, SerializeDisplay)]
pub enum ExCode {
    #[serde(rename = "00")]
    None,
    #[serde(rename = "01")]
    ExRights, // 권리락
    #[serde(rename = "02")]
    ExDividend, // 배당락
    #[serde(rename = "03")]
    ExEtfDividend, // 분배락
    #[serde(rename = "04")]
    ExRightsAndDividend, // 권배락
    #[serde(rename = "05")]
    MidOrQtrExDividend, // 중간(분기)배당락
    #[serde(rename = "06")]
    MidExRightsAndDividend, // 권리중간배당락
    #[serde(rename = "07")]
    QtrExRightsAndDividend, // 권리분기배당락
}
impl std::fmt::Display for ExCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::None => "00",
            Self::ExRights => "01",
            Self::ExDividend => "02",
            Self::ExEtfDividend => "03",
            Self::ExRightsAndDividend => "04",
            Self::MidOrQtrExDividend => "05",
            Self::MidExRightsAndDividend => "06",
            Self::QtrExRightsAndDividend => "07",
        })
    }
}

#[derive(Clone, Debug, Deserialize, SerializeDisplay)]
pub enum BelongClassCode {
    #[serde(rename = "0")]
    MeanVolume, // 평균거래량
    #[serde(rename = "1")]
    IncreasedVolumeRate, // 거래증가율
    #[serde(rename = "2")]
    MeanRotationRate, // 평균거래회전율
    #[serde(rename = "3")]
    Amount, // 거래금액순
    #[serde(rename = "4")]
    AmountRotationRate, // 평균거래금액회전율
}

impl std::fmt::Display for BelongClassCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::MeanVolume => "0",
            Self::IncreasedVolumeRate => "1",
            Self::MeanRotationRate => "2",
            Self::Amount => "3",
            Self::AmountRotationRate => "4",
        })
    }
}

#[derive(Debug, Clone, Copy, SerializeDisplay)]
pub struct TargetClassCode {
    pub margin_30: bool,
    pub margin_40: bool,
    pub margin_50: bool,
    pub margin_60: bool,
    pub margin_100: bool,
    pub credit_30: bool,
    pub credit_40: bool,
    pub credit_50: bool,
    pub credit_60: bool,
}

impl std::fmt::Display for TargetClassCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}{}{}{}{}{}{}{}",
            if self.margin_30 { '1' } else { '0' },
            if self.margin_40 { '1' } else { '0' },
            if self.margin_50 { '1' } else { '0' },
            if self.margin_60 { '1' } else { '0' },
            if self.margin_100 { '1' } else { '0' },
            if self.credit_30 { '1' } else { '0' },
            if self.credit_40 { '1' } else { '0' },
            if self.credit_50 { '1' } else { '0' },
            if self.credit_60 { '1' } else { '0' },
        ))
    }
}

#[derive(Debug, Clone, Copy, SerializeDisplay)]
pub struct TargetExeceptClassCode {
    pub overheat: bool,             // 투자위험/경고/주의
    pub administrated: bool,        // 관리종목
    pub settlement_trading: bool,   // 정리매매
    pub insufficient_posting: bool, // 불성실공시
    pub preferred_share: bool,      // 우선주
    pub suspended: bool,            // 거래정지
}

impl std::fmt::Display for TargetExeceptClassCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}{}{}{}{}",
            if self.overheat { '1' } else { '0' },
            if self.administrated { '1' } else { '0' },
            if self.settlement_trading { '1' } else { '0' },
            if self.insufficient_posting { '1' } else { '0' },
            if self.preferred_share { '1' } else { '0' },
            if self.suspended { '1' } else { '0' },
        ))
    }
}

#[derive(Clone, Debug, Deserialize, SerializeDisplay)]
pub enum ShareClassCode {
    #[serde(rename = "0")]
    Whole,
    #[serde(rename = "1")]
    Common,
    #[serde(rename = "2")]
    Preferred,
}

impl std::fmt::Display for ShareClassCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Whole => "0",
            Self::Common => "1",
            Self::Preferred => "2",
        })
    }
}

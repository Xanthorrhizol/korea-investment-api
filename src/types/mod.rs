pub mod config;
mod crypto;
pub mod request;
pub mod response;
pub mod stream;
mod time;

use crate::Error;
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum OrderClass {
    #[default]
    /// 지정가
    Limit,
    /// 시장가
    Market,
    /// 조건부지정가
    ConditionalLimit,
    /// 최유리지정가
    Best,
    /// 최우선지정가
    First,
    /// 장전시간외
    PreMarket,
    /// 장후시간외
    PostMarket,
    /// 시간외단일가
    OutMarketSinglePrice,
    /// 자기주식
    MyStock,
    /// 자기주식S-Option
    MyStockSOption,
    /// 자시주식금전신탁
    MyStockMoneyTrust,
    /// IOC지정가(즉시체결, 잔량취소)
    IOCLimit,
    /// FOK지정가(즉시체결, 잔량취소)
    FOKLimit,
    /// IOC시장가(즉시체결, 잔량취소)
    IOCMarket,
    /// FOK시장가(즉시체결, 잔량취소)
    FOKMarket,
    /// IOC최유리(즉시체결, 잔량취소)
    IOCBest,
    /// FOK최유리(즉시체결, 잔량취소)
    FOKBest,
    /// 장중대량(즉시체결, 잔량취소)
    MidMarketMassive,
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
            Self::MidMarketMassive => "51".to_string(),
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
            "51" => OrderClass::MidMarketMassive,
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
    #[serde(rename = "CTPF1002R")]
    BasicStockInfo,
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
            TrId::BasicStockInfo => "CTPF1002R",
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
            "CTPF1002R" => TrId::BasicStockInfo,
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
            "8" => What::BuyIn,
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

/// 시장ID코드
#[derive(Clone, Debug, Deserialize, SerializeDisplay)]
pub enum MarketId {
    #[serde(rename = "AGR")]
    /// AGR.농축산물파생
    Agricultural,
    #[serde(rename = "BON")]
    /// BON.채권파생
    Bond,
    #[serde(rename = "CMD")]
    /// CMD.일반상품시장
    Commodity,
    #[serde(rename = "CUR")]
    /// CUR.통화파생
    Currency,
    #[serde(rename = "ENG")]
    /// ENG.에너지파생
    Energy,
    #[serde(rename = "EQU")]
    /// EQU.주식파생
    Equity,
    #[serde(rename = "ETF")]
    /// ETF.ETF파생
    Etf,
    #[serde(rename = "IRT")]
    /// IRT.금리파생
    InterestRate,
    #[serde(rename = "KNX")]
    /// KNX.코넥스
    Konex,
    #[serde(rename = "KSQ")]
    /// KSQ.코스닥
    Kosdaq,
    #[serde(rename = "MTL")]
    /// MTL.금속파생
    Metal,
    #[serde(rename = "SPI")]
    /// SPI.주가지수파생
    StockPriceIndex,
    #[serde(rename = "STK")]
    /// STK.유가증권
    Stock,
}
impl std::fmt::Display for MarketId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            MarketId::Agricultural => "AGR",
            MarketId::Bond => "BON",
            MarketId::Commodity => "CMD",
            MarketId::Currency => "CUR",
            MarketId::Energy => "ENG",
            MarketId::Equity => "EQU",
            MarketId::Etf => "ETF",
            MarketId::InterestRate => "IRT",
            MarketId::Konex => "KNX",
            MarketId::Kosdaq => "KSQ",
            MarketId::Metal => "MTL",
            MarketId::StockPriceIndex => "SPI",
            MarketId::Stock => "STK",
        })
    }
}

impl std::str::FromStr for MarketId {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "AGR" => Self::Agricultural,
            "BON" => Self::Bond,
            "CMD" => Self::Commodity,
            "CUR" => Self::Currency,
            "ENG" => Self::Energy,
            "EQU" => Self::Equity,
            "ETF" => Self::Etf,
            "IRT" => Self::InterestRate,
            "KNX" => Self::Konex,
            "KSQ" => Self::Kosdaq,
            "MTL" => Self::Metal,
            "SPI" => Self::StockPriceIndex,
            "STK" => Self::Stock,
            _ => {
                return Err(Error::BrokenProtocol("MarketId", s.to_string()));
            }
        })
    }
}

/// 증권그룹ID코드
#[derive(Clone, Debug, Deserialize, SerializeDisplay)]
pub enum SecurityGroupId {
    #[serde(rename = "BC")]
    /// BC.수익증권
    Income,
    #[serde(rename = "DR")]
    /// DR.주식예탁증서
    StockDepositaryReceipt,
    #[serde(rename = "EF")]
    /// EF.ETF
    Etf,
    #[serde(rename = "EN")]
    /// EN.ETN
    Etn,
    #[serde(rename = "EW")]
    /// EW.ELW
    Elw,
    #[serde(rename = "FE")]
    /// FE.해외ETF
    ForeignEtf,
    #[serde(rename = "FO")]
    /// FO.선물옵션
    FutureOption,
    #[serde(rename = "FS")]
    /// FS.외국주권
    ForeignStock,
    #[serde(rename = "FU")]
    /// FU.선물
    Future,
    #[serde(rename = "FX")]
    /// FX.플렉스 선물
    FlexFuture,
    #[serde(rename = "GD")]
    /// GD.금현물
    Gold,
    #[serde(rename = "IC")]
    /// IC.투자계약증권
    InvestmentContract,
    #[serde(rename = "IF")]
    /// IF.사회간접자본투융자회사
    IndirectCapitalInvestmentCompany,
    #[serde(rename = "KN")]
    /// KN.코넥스주권
    KonexStock,
    #[serde(rename = "MF")]
    /// MF.투자회사
    InvestmentCompany,
    #[serde(rename = "OP")]
    /// OP.옵션
    Option,
    #[serde(rename = "RT")]
    /// RT.부동산투자회사
    RealEstateInvestmentCompany,
    #[serde(rename = "SC")]
    /// SC.선박투자회사
    ShipInvestmentCompany,
    #[serde(rename = "SR")]
    /// SR.신주인수권증서
    Warrant,
    #[serde(rename = "ST")]
    /// ST.주권
    Stock,
    #[serde(rename = "SW")]
    /// SW.신주인수권증권
    WarrantSecurity,
    #[serde(rename = "TC")]
    /// TC.신탁수익증권
    TrustIncome,
}

impl std::fmt::Display for SecurityGroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            SecurityGroupId::Income => "BC",
            SecurityGroupId::StockDepositaryReceipt => "DR",
            SecurityGroupId::Etf => "EF",
            SecurityGroupId::Etn => "EN",
            SecurityGroupId::Elw => "EW",
            SecurityGroupId::ForeignEtf => "FE",
            SecurityGroupId::FutureOption => "FO",
            SecurityGroupId::ForeignStock => "FS",
            SecurityGroupId::Future => "FU",
            SecurityGroupId::FlexFuture => "FX",
            SecurityGroupId::Gold => "GD",
            SecurityGroupId::InvestmentContract => "IC",
            SecurityGroupId::IndirectCapitalInvestmentCompany => "IF",
            SecurityGroupId::KonexStock => "KN",
            SecurityGroupId::InvestmentCompany => "MF",
            SecurityGroupId::Option => "OP",
            SecurityGroupId::RealEstateInvestmentCompany => "RT",
            SecurityGroupId::ShipInvestmentCompany => "SC",
            SecurityGroupId::Warrant => "SR",
            SecurityGroupId::Stock => "ST",
            SecurityGroupId::WarrantSecurity => "SW",
            SecurityGroupId::TrustIncome => "TC",
        })
    }
}

impl std::str::FromStr for SecurityGroupId {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "BC" => Self::Income,
            "DR" => Self::StockDepositaryReceipt,
            "EF" => Self::Etf,
            "EN" => Self::Etn,
            "EW" => Self::Elw,
            "FE" => Self::ForeignEtf,
            "FO" => Self::FutureOption,
            "FS" => Self::ForeignStock,
            "FU" => Self::Future,
            "FX" => Self::FlexFuture,
            "GD" => Self::Gold,
            "IC" => Self::InvestmentContract,
            "IF" => Self::IndirectCapitalInvestmentCompany,
            "KN" => Self::KonexStock,
            "MF" => Self::InvestmentCompany,
            "OP" => Self::Option,
            "RT" => Self::RealEstateInvestmentCompany,
            "SC" => Self::ShipInvestmentCompany,
            "SR" => Self::Warrant,
            "ST" => Self::Stock,
            "SW" => Self::WarrantSecurity,
            "TC" => Self::TrustIncome,
            _ => {
                return Err(Error::BrokenProtocol("SecurityGroupId", s.to_string()));
            }
        })
    }
}

/// 거래소구분코드
#[derive(Debug, Clone, Deserialize, SerializeDisplay)]
pub enum ExchangeCode {
    #[serde(rename = "01")]
    /// 01.한국증권
    KoreaSecurities,
    #[serde(rename = "02")]
    /// 02.증권거래소
    SecuritiesExchange,
    #[serde(rename = "03")]
    /// 03.코스닥
    Kosdaq,
    #[serde(rename = "04")]
    /// 04.K-OTC
    KOTC,
    #[serde(rename = "05")]
    /// 05.선물거래소
    FutureExchange,
    #[serde(rename = "06")]
    /// 06.CME
    CME,
    #[serde(rename = "07")]
    /// 07.EUREX
    EUREX,
    #[serde(rename = "21")]
    /// 21.금현물
    Gold,
    #[serde(rename = "50")]
    /// 50.미국주간
    USWeekly,
    #[serde(rename = "51")]
    /// 51.홍콩
    HongKong,
    #[serde(rename = "52")]
    /// 52.상해B
    ShanghaiB,
    #[serde(rename = "53")]
    /// 53.심천
    Shenzhen,
    #[serde(rename = "54")]
    /// 54.홍콩거래소
    HongKongExchange,
    #[serde(rename = "55")]
    /// 55.미국
    US,
    #[serde(rename = "56")]
    /// 56.일본
    Japan,
    #[serde(rename = "57")]
    /// 57.상해A
    ShanghaiA,
    #[serde(rename = "58")]
    /// 58.심천A
    ShenzhenA,
    #[serde(rename = "59")]
    /// 59.베트남
    Vietnam,
    #[serde(rename = "61")]
    /// 61.장전시간외시장
    PreMarket,
    #[serde(rename = "64")]
    /// 64.경쟁대량매매
    CompetitiveMassive,
    #[serde(rename = "65")]
    /// 65.경매매시장
    AuctionMarket,
    #[serde(rename = "81")]
    /// 81.시간외단일가시장
    OutMarketSinglePrice,
}

impl std::fmt::Display for ExchangeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            ExchangeCode::KoreaSecurities => "01",
            ExchangeCode::SecuritiesExchange => "02",
            ExchangeCode::Kosdaq => "03",
            ExchangeCode::KOTC => "04",
            ExchangeCode::FutureExchange => "05",
            ExchangeCode::CME => "06",
            ExchangeCode::EUREX => "07",
            ExchangeCode::Gold => "21",
            ExchangeCode::USWeekly => "50",
            ExchangeCode::HongKong => "51",
            ExchangeCode::ShanghaiB => "52",
            ExchangeCode::Shenzhen => "53",
            ExchangeCode::HongKongExchange => "54",
            ExchangeCode::US => "55",
            ExchangeCode::Japan => "56",
            ExchangeCode::ShanghaiA => "57",
            ExchangeCode::ShenzhenA => "58",
            ExchangeCode::Vietnam => "59",
            ExchangeCode::PreMarket => "61",
            ExchangeCode::CompetitiveMassive => "64",
            ExchangeCode::AuctionMarket => "65",
            ExchangeCode::OutMarketSinglePrice => "81",
        })
    }
}

impl std::str::FromStr for ExchangeCode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "01" => Self::KoreaSecurities,
            "02" => Self::SecuritiesExchange,
            "03" => Self::Kosdaq,
            "04" => Self::KOTC,
            "05" => Self::FutureExchange,
            "06" => Self::CME,
            "07" => Self::EUREX,
            "21" => Self::Gold,
            "50" => Self::USWeekly,
            "51" => Self::HongKong,
            "52" => Self::ShanghaiB,
            "53" => Self::Shenzhen,
            "54" => Self::HongKongExchange,
            "55" => Self::US,
            "56" => Self::Japan,
            "57" => Self::ShanghaiA,
            "58" => Self::ShenzhenA,
            "59" => Self::Vietnam,
            "61" => Self::PreMarket,
            "64" => Self::CompetitiveMassive,
            "65" => Self::AuctionMarket,
            "81" => Self::OutMarketSinglePrice,
            _ => {
                return Err(Error::BrokenProtocol("ExchangeCode", s.to_string()));
            }
        })
    }
}

/// 주식종류코드
#[derive(Debug, Clone, Deserialize, SerializeDisplay)]
pub enum StockKindCode {
    #[serde(rename = "000")]
    /// 000.해당사항없음
    None,
    #[serde(rename = "101")]
    /// 101.보통주
    Common,
    #[serde(rename = "201")]
    /// 201.우선주
    Preferred,
    #[serde(rename = "202")]
    /// 202.2우선주
    Preferred2,
    #[serde(rename = "203")]
    /// 203.3우선주
    Preferred3,
    #[serde(rename = "204")]
    /// 204.4우선주
    Preferred4,
    #[serde(rename = "205")]
    /// 205.5우선주
    Preferred5,
    #[serde(rename = "206")]
    /// 206.6우선주
    Preferred6,
    #[serde(rename = "207")]
    /// 207.7우선주
    Preferred7,
    #[serde(rename = "208")]
    /// 208.8우선주
    Preferred8,
    #[serde(rename = "209")]
    /// 209.9우선주
    Preferred9,
    #[serde(rename = "210")]
    /// 210.10우선주
    Preferred10,
    #[serde(rename = "211")]
    /// 211.11우선주
    Preferred11,
    #[serde(rename = "212")]
    /// 212.12우선주
    Preferred12,
    #[serde(rename = "213")]
    /// 213.13우선주
    Preferred13,
    #[serde(rename = "214")]
    /// 214.14우선주
    Preferred14,
    #[serde(rename = "215")]
    /// 215.15우선주
    Preferred15,
    #[serde(rename = "216")]
    /// 216.16우선주
    Preferred16,
    #[serde(rename = "217")]
    /// 217.17우선주
    Preferred17,
    #[serde(rename = "218")]
    /// 218.18우선주
    Preferred18,
    #[serde(rename = "219")]
    /// 219.19우선주
    Preferred19,
    #[serde(rename = "220")]
    /// 220.20우선주
    Preferred20,
    #[serde(rename = "301")]
    /// 301.후배주
    After,
    #[serde(rename = "401")]
    /// 401.혼합주
    Mixed,
}

impl std::fmt::Display for StockKindCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            StockKindCode::None => "000",
            StockKindCode::Common => "101",
            StockKindCode::Preferred => "201",
            StockKindCode::Preferred2 => "202",
            StockKindCode::Preferred3 => "203",
            StockKindCode::Preferred4 => "204",
            StockKindCode::Preferred5 => "205",
            StockKindCode::Preferred6 => "206",
            StockKindCode::Preferred7 => "207",
            StockKindCode::Preferred8 => "208",
            StockKindCode::Preferred9 => "209",
            StockKindCode::Preferred10 => "210",
            StockKindCode::Preferred11 => "211",
            StockKindCode::Preferred12 => "212",
            StockKindCode::Preferred13 => "213",
            StockKindCode::Preferred14 => "214",
            StockKindCode::Preferred15 => "215",
            StockKindCode::Preferred16 => "216",
            StockKindCode::Preferred17 => "217",
            StockKindCode::Preferred18 => "218",
            StockKindCode::Preferred19 => "219",
            StockKindCode::Preferred20 => "220",
            StockKindCode::After => "301",
            StockKindCode::Mixed => "401",
        })
    }
}

impl std::str::FromStr for StockKindCode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "000" => Self::None,
            "101" => Self::Common,
            "201" => Self::Preferred,
            "202" => Self::Preferred2,
            "203" => Self::Preferred3,
            "204" => Self::Preferred4,
            "205" => Self::Preferred5,
            "206" => Self::Preferred6,
            "207" => Self::Preferred7,
            "208" => Self::Preferred8,
            "209" => Self::Preferred9,
            "210" => Self::Preferred10,
            "211" => Self::Preferred11,
            "212" => Self::Preferred12,
            "213" => Self::Preferred13,
            "214" => Self::Preferred14,
            "215" => Self::Preferred15,
            "216" => Self::Preferred16,
            "217" => Self::Preferred17,
            "218" => Self::Preferred18,
            "219" => Self::Preferred19,
            "220" => Self::Preferred20,
            "301" => Self::After,
            "401" => Self::Mixed,
            _ => {
                return Err(Error::BrokenProtocol("StockKindCode", s.to_string()));
            }
        })
    }
}

/// 상품유형코드
#[derive(Debug, Clone, Deserialize, SerializeDisplay, Default)]
pub enum ProductTypeCode {
    #[default]
    #[serde(rename = "300")]
    /// 300: 주식, ETF, ETN, ELW
    Stock,
    #[serde(rename = "301")]
    /// 301: 선물옵션
    FutureOption,
    #[serde(rename = "302")]
    /// 302: 채권
    Bond,
    #[serde(rename = "306")]
    /// 306: ELS
    ELS,
}

impl std::fmt::Display for ProductTypeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            ProductTypeCode::Stock => "300",
            ProductTypeCode::FutureOption => "301",
            ProductTypeCode::Bond => "302",
            ProductTypeCode::ELS => "306",
        })
    }
}

impl std::str::FromStr for ProductTypeCode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "300" => Self::Stock,
            "301" => Self::FutureOption,
            "302" => Self::Bond,
            "306" => Self::ELS,
            _ => {
                return Err(Error::BrokenProtocol("ProductTypeCode", s.to_string()));
            }
        })
    }
}

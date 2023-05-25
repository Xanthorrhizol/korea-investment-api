/// 체결구분
pub enum ExecClass {
    All,     // 전체(00)
    Partial, // 체결(01)
    Nothing, // 미체결(02)
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
pub enum HourClassCode {
    InMarket,                    // 장중(0)
    PostMarketPrediction,        // 장후예상(A)
    PreMarketPrediction,         // 장전예상(B)
    PostNinePmPredictionOrVi,    // 9시이후의 예상가, VI발동(C)
    OutMarketSinglePricePredict, // 시간외 단일가 예상(D)
}

impl From<String> for HourClassCode {
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

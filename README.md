# 한국투자증권 API for Rust

## 현재 지원되는 기능

- 인증
- 국내 주식 주문
  - 현금주문
  - 정정/취소
- 국내 주식 시세
  - 일자별
- 실시간 시세
  - 체결
  - 호가
  - 체결통보

## 사용 방법

- 구조체 생성

```rust
use korea_investment_api::{KoreaInvestmentApi, Environment, Account, Error};

static APPKEY: &'static str = "some-app-key";
static APPSECRET: &'static str = "some-app-secret";
static CANO: &'static str = "12345678";
static ACNT_PRDT_CD: &'static str = "01";
static HTS_ID: &'static str = "my-hts-id";

async fn get_api() -> Result<KoreaInvestmentApi, Error> {
    let account = Account {
        cano: CANO.to_string(),
        acnt_prdt_cd: ACNT_PRDT_CD.to_string(),
    };
    KoreaInvestmentApi::new(
        Environment::Virtual, // 실전투자: Real, 모의투자: Virtual
        APPKEY.to_string(),
        APPSECRET.to_string(),
        account,
        hts_id: HTS_ID.to_string(),
    ).await
}
```

- API 사용

```rust
use korea_investment_api::types::{MarketCode, PeriodCode};

#[tokio::main]
async fn main() {
    let api = get_api().await.unwrap();
    let samsung_electronics_daily_prices = api.quote.daily_price(
        MarketCode::Stock, // 주식(Stock) vs ETF(Etf)
        "005930".to_string(), // 6자리 종목 코드
        PeriodCode::ThirtyDays, // 기간 코드(ThirtyDays(30일), ThirtyWeeks(30주), ThirtyMonths(30달))
    ).await;
    ...
}
```

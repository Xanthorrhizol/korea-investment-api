# 한국투자증권 API for Rust

> 💡필요한 부분이나 이슈가 있다면 issue 올려주시면 그 부분 볼 수 있도록 하겠습니다.  
> 💡cargo에 publish한 라이브러리를 이용하거나, Release에 올라온 버전을 이용 바랍니다.

## 현재 지원되는 기능

- 인증
- 국내 주식 주문
  - 현금주문
  - 신용주문
  - 정정/취소
  - 일별 주문체결 조회
  - 잔고 조회
  - 정정취소가능주문 조회
  - 매수가능조회
- 국내 주식 시세
  - 주식 일자별 시세
  - 거래량 순위
  - 관심종목 그룹조회
  - 관심종목 그룹별 종목조회
  - 주식기본조회
- 실시간 시세
  - 체결(KRX/NXT/통합)
  - 호가(KRX/NXT/통합)
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
static REAL_APPKEY: &'static str = "some-real-app-key"; // 실전투자 환경만 이용 가능한 API 이용을 위한 APPKEY
static REAL_APPSECRET: &'static str = "some-real-app-secret"; // 실전투자 환경만 이용 가능한 API 이용을 위한 APPSECRET

async fn get_api() -> Result<KoreaInvestmentApi, Error> {
    let account = Account {
        cano: CANO.to_string(),
        acnt_prdt_cd: ACNT_PRDT_CD.to_string(),
    };
    KoreaInvestmentApi::new(
        Environment::Virtual, // 실전투자: Real, 모의투자: Virtual
        APPKEY,
        APPSECRET,
        account,
        HTS_ID,
        None, // token
        None, // approval_key
        Some(REAL_APPKEY), // Optional
        Some(REAL_APPSECRET), // Optional
        None, // real_token
        None, // real_approval_key
    ).await
}
```

- API 사용

```rust
use korea_investment_api::types::{MarketCode, PeriodCode, TrId};

#[tokio::main]
async fn main() {
    let api = get_api().await.unwrap();

    // 삼성전자 일자별 가격(단일 API 호출)
    let samsung_electronics_daily_prices = api.quote.daily_price(
        MarketCode::Stock, // 주식(Stock) vs ETF(Etf)
        "005930", // 6자리 종목 코드
        PeriodCode::ThirtyDays, // 기간 코드(ThirtyDays(30일), ThirtyWeeks(30주), ThirtyMonths(30달))
        false, // 수정주가 원주가 가격 여부(수정주가 반영: true)
    ).await;

    // 신규 주문
    let order_result = api
        .order
        .order_cash(
            OrderClass::Limit,
            Direction::Bid,
            "005930",
            Quantity::from(1),
            Price::from(50_000),
            Some(TargetExchange::KRX),
        )
        .await;
    println!("신규 주문 Response: {:?}", order_result);
    
    // 삼성전자 호가 실시간 시세 구독
    let (rx, subscribe_response) = api.k_data.subscribe_market("005930", TrId::RealtimeOrdbKrx).unwrap();

    // 구독한 시세 읽기
    while let Ok(ordb) = rx.recv().await {
        println!("Got orderbook: {:?}", ordb);
    }
}
```

> :bulb: 메시지가 유실되는 케이스 발생 시, 환경변수 CHANNEL_SIZE를 더 높은 갚으로 설정해 주시기 바랍니다. 기본값은 `16384`입니다.

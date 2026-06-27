# 빠른 시작

## 편의 임포트 (prelude)

자주 쓰는 타입을 한 번에 가져오려면 `prelude`를 사용하세요.

```rust
use korea_investment_api::prelude::*;
```

`prelude`는 `KoreaInvestmentApi`, `Config`, `types::*`(주문/시세 관련 enum 등), 요청 모듈(`order` / `quote` / `subscribe`), 실시간 스트림 타입(`Exec` / `ExecBody` / `Ordb` / `OrdbBody`)을 재노출합니다.

> `Error` 타입은 크레이트 루트에 있으므로 `prelude`에 포함되지 않습니다. 필요하면 `use korea_investment_api::Error;`로 따로 가져오세요.

## API 인스턴스 생성

```rust
use korea_investment_api::{KoreaInvestmentApi, Error};
use korea_investment_api::types::{Environment, Account};

async fn create_api() -> Result<KoreaInvestmentApi, Error> {
    let account = Account {
        cano: "12345678".to_string(),        // 계좌번호 앞 8자리
        acnt_prdt_cd: "01".to_string(),      // 계좌번호 뒤 2자리
    };

    KoreaInvestmentApi::new(
        Environment::Virtual,  // 실전: Real, 모의: Virtual
        "your-app-key",
        "your-app-secret",
        account,
        "your-hts-id",
        None,  // token: None이면 자동 발급
        None,  // approval_key: None이면 자동 발급
        Some("your-real-app-key".to_string()),     // 실전투자 전용 API용
        Some("your-real-app-secret".to_string()),  // 실전투자 전용 API용
        None,  // real_token
        None,  // real_approval_key
    ).await
}
```

> `token`과 `approval_key`에 `Some(...)` 값을 전달하면 API 호출을 건너뜁니다. 캐싱된 값이 있을 때 활용하세요.

## 기본 사용 예시

### 일자별 시세 조회

```rust
use korea_investment_api::types::{MarketCode, PeriodCode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = create_api().await?;

    // 삼성전자 30일 시세
    let prices = api.quote.daily_price(
        MarketCode::Stock,
        "005930",            // 종목 코드
        PeriodCode::ThirtyDays,
        false,               // 수정주가 여부 (true = 수정주가 반영)
    ).await?;

    println!("{:?}", prices);
    Ok(())
}
```

### 현금 매수 주문

```rust
use korea_investment_api::types::{Direction, OrderClass, Price, Quantity};

let result = api.order.order_cash(
    OrderClass::Limit,      // 지정가
    Direction::Bid,         // 매수
    "005930",               // 삼성전자
    Quantity::from(10u32),  // 10주
    Price::from(70000u32),  // 70,000원
    None,                   // 거래소 코드 (None = 기본값)
).await?;
```

### 실시간 체결 구독

```rust
use korea_investment_api::prelude::*;  // Exec, ExecBody, TrId 포함

// KRX 체결 스트림 구독
let (rx, response) = api.k_data.subscribe_market::<Exec, ExecBody>(
    "005930",
    TrId::RealtimeExecKrx,
).await?;

if let Some(mut rx) = rx {
    // recv()는 broadcast Receiver이므로 Result를 반환합니다
    while let Ok(exec) = rx.recv().await {
        println!("체결: {:?}", exec);
    }
}
```

### 토큰 저장

```rust
// 프로그램 종료 전 토큰 캐싱
api.export_config(&config)?;
```

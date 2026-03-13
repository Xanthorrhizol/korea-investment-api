# 시세 (Quote)

`api.quote`는 국내 주식 시세 조회 API를 제공합니다. `src/stock/quote.rs`에 구현되어 있습니다.

> **주의:** 거래량순위, 관심종목, 주식기본조회 API는 실전투자 엔드포인트만 지원합니다. 모의투자 환경에서도 실전투자 서버로 요청이 전송됩니다.

---

## daily_price — 일자별 시세 `[v1_국내주식-010]`

종목의 일자별 / 주별 / 월별 시세를 조회합니다.

```rust
pub async fn daily_price(
    &self,
    market_code: MarketCode,
    shortcode: &str,
    period_code: PeriodCode,
    is_adjust_price: bool,
) -> Result<response::stock::quote::DailyPriceResponse, Error>
```

### 파라미터

| 파라미터 | 타입 | 설명 |
|---|---|---|
| `market_code` | `MarketCode` | `Stock` (주식) / `Etf` (ETF) |
| `shortcode` | `&str` | 종목 코드 (6자리) |
| `period_code` | `PeriodCode` | 기간 코드 (아래 표 참고) |
| `is_adjust_price` | `bool` | `true` = 수정주가 반영 |

### PeriodCode 값

| 값 | 설명 |
|---|---|
| `ThirtyDays` | 최근 30일 (일별) |
| `ThirtyWeeks` | 최근 30주 (주별) |
| `ThirtyMonths` | 최근 30개월 (월별) |

### 예시

```rust
use korea_investment_api::types::{MarketCode, PeriodCode};

// 삼성전자 30일 일자별 시세 (수정주가 미반영)
let prices = api.quote.daily_price(
    MarketCode::Stock,
    "005930",
    PeriodCode::ThirtyDays,
    false,
).await?;
```

---

## volume_rank — 거래량 순위 `[v1_국내주식-047]`

거래량 기준 종목 순위를 조회합니다. **실전투자 전용입니다.**

```rust
pub async fn volume_rank(
    &self,
    params: request::stock::quote::VolumeRankParameter,
) -> Result<response::stock::quote::VolumeRankResponse, Error>
```

### 예시

```rust
use korea_investment_api::types::request::stock::quote::VolumeRankParameter;

let params = VolumeRankParameter::new(/* ... */);
let ranks = api.quote.volume_rank(params).await?;
```

---

## group_item — 관심종목 그룹별 종목조회 `[국내주식-203]`

관심종목 그룹에 속한 종목 목록을 조회합니다. **실전투자 전용입니다.**

```rust
pub async fn group_item(
    &self,
    params: request::stock::quote::GroupItemParameter,
) -> Result<response::stock::quote::GroupItemResponse, Error>
```

---

## group_list — 관심종목 그룹조회 `[국내주식-204]`

관심종목 그룹 목록을 조회합니다. **실전투자 전용입니다.**

```rust
pub async fn group_list(
    &self,
    params: request::stock::quote::GroupListParameter,
) -> Result<response::stock::quote::GroupListResponse, Error>
```

---

## basic_stock_info — 주식 기본 정보 `[v1_국내주식-067]`

종목의 기본 정보를 조회합니다. **실전투자 전용입니다.**

```rust
pub async fn basic_stock_info(
    &self,
    prdt_type_cd: ProductTypeCode,
    pdno: &str,
) -> Result<response::stock::quote::BasicStockInfoResponse, Error>
```

### 파라미터

| 파라미터 | 타입 | 설명 |
|---|---|---|
| `prdt_type_cd` | `ProductTypeCode` | 상품 유형 코드 |
| `pdno` | `&str` | 종목 코드 |

### 예시

```rust
use korea_investment_api::types::ProductTypeCode;

let info = api.quote.basic_stock_info(
    ProductTypeCode::Stock,
    "005930",
).await?;
```

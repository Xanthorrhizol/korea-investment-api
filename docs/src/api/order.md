# 주문 (Order)

`api.order`는 국내 주식 주문 관련 API를 제공합니다. `src/stock/order.rs`에 구현되어 있습니다.

---

## order_cash — 현금 주문 `[v1_국내주식-001]`

현금으로 주식을 매수 또는 매도합니다.

```rust
pub async fn order_cash(
    &self,
    order_division: OrderClass,
    order_direction: Direction,
    pdno: &str,
    qty: Quantity,
    price: Price,
    excg_id_dvsn_cd: Option<TargetExchange>,
) -> Result<response::stock::order::Body::Order, Error>
```

### 파라미터

| 파라미터 | 타입 | 설명 |
|---|---|---|
| `order_division` | `OrderClass` | 주문 유형 (지정가, 시장가 등) |
| `order_direction` | `Direction` | `Bid` (매수) / `Ask` (매도) |
| `pdno` | `&str` | 종목 코드 (6자리, 예: `"005930"`) |
| `qty` | `Quantity` | 주문 수량 |
| `price` | `Price` | 주문 단가 (시장가일 경우 `0`) |
| `excg_id_dvsn_cd` | `Option<TargetExchange>` | 거래소 코드 (`None` = 기본값) |

### 예시

```rust
use korea_investment_api::types::{Direction, OrderClass, Price, Quantity};

// 삼성전자 10주 지정가 매수 (70,000원)
let result = api.order.order_cash(
    OrderClass::Limit,
    Direction::Bid,
    "005930",
    Quantity::from(10u32),
    Price::from(70000u32),
    None,
).await?;
```

### 지원 환경

실전 · 모의 모두 지원합니다.

---

## order_credit — 신용 주문 `[v1_국내주식-002]`

신용으로 주식을 매수 또는 매도합니다.

```rust
pub async fn order_credit(
    &self,
    order_division: OrderClass,
    order_direction: Direction,
    pdno: &str,
    qty: Quantity,
    price: Price,
    excg_id_dvsn_cd: Option<TargetExchange>,
    credit_type: CreditType,
) -> Result<response::stock::order::Body::Order, Error>
```

> **주의:** 실전투자 환경만 지원합니다. 모의투자에서 호출하면 `Error::NotSupported`가 반환됩니다.

---

## correct — 정정/취소 `[v1_국내주식-003]`

미체결 주문을 정정하거나 취소합니다.

```rust
pub async fn correct(
    &self,
    order_division: OrderClass,
    krx_fwdg_ord_orgno: &str,
    orgn_odno: &str,
    rvse_cncl_dvsn_cd: CorrectionClass,
    qty_all_ord_yn: bool,
    qty: Quantity,
    price: Price,
    excg_id_dvsn_cd: Option<TargetExchange>,
) -> Result<response::stock::order::Body::Order, Error>
```

### 파라미터

| 파라미터 | 타입 | 설명 |
|---|---|---|
| `krx_fwdg_ord_orgno` | `&str` | KRX 전송 주문 기관 번호 |
| `orgn_odno` | `&str` | 원주문번호 |
| `rvse_cncl_dvsn_cd` | `CorrectionClass` | `Correct` (정정) / `Cancel` (취소) |
| `qty_all_ord_yn` | `bool` | 전량 주문 여부 |
| `qty` | `Quantity` | 정정/취소 수량 (전량이면 `0`) |
| `price` | `Price` | 정정 단가 (취소 시 `0`) |

---

## inquire_daily_ccld — 일별 주문체결 조회 `[v1_국내주식-005]`

기간별 주문 및 체결 내역을 조회합니다.

```rust
pub async fn inquire_daily_ccld(
    &self,
    inqr_strt_dt: &str,   // 조회 시작일 "YYYYMMDD"
    inqr_end_dt: &str,    // 조회 종료일 "YYYYMMDD"
    sll_buy_dvsn_cd: Option<String>,
    inqr_dvsn: Option<String>,
    pdno: Option<String>,
    ccld_dvsn: Option<String>,
    ord_gno_brno: Option<String>,
    odno: Option<String>,
    inqr_dvsn_3: Option<String>,
    inqr_dvsn_1: Option<String>,
    ctx_area_fk100: Option<String>,  // 연속조회 키
    ctx_area_nk100: Option<String>,  // 연속조회 키
    excg_id_dvsn_cd: Option<TargetExchange>,
) -> Result<response::stock::order::daily_ccld::InquireDailyCcld, Error>
```

### 연속조회

한 번에 반환되는 건수가 최대치(실전 50건, 모의 20건)를 초과하면 응답의 `ctx_area_fk100` / `ctx_area_nk100` 값을 다음 요청에 전달합니다.

### TR ID 자동 선택

조회 시작일에 따라 TR ID가 자동으로 선택됩니다:
- **3개월 이내:** Recent TR 사용
- **3개월 이전:** Past TR 사용

---

## inquire_balance — 잔고 조회 `[v1_국내주식-006]`

계좌 잔고를 조회합니다.

```rust
pub async fn inquire_balance(
    &self,
    afhr_flpr_yn: Option<String>,    // 시간외 단일가 여부
    inqr_dvsn: Option<String>,       // 조회 구분
    prcs_dvsn: Option<String>,       // 처리 구분
    ctx_area_fk100: Option<String>,  // 연속조회 키
    ctx_area_nk100: Option<String>,  // 연속조회 키
) -> Result<response::stock::order::balance::InquireBalance, Error>
```

---

## inquire_psbl_rvsecncl — 정정취소가능주문 조회 `[v1_국내주식-004]`

정정 또는 취소가 가능한 미체결 주문 목록을 조회합니다.

```rust
pub async fn inquire_psbl_rvsecncl(
    &self,
    ctx_area_fk100: Option<String>,
    ctx_area_nk100: Option<String>,
    inqr_dvsn_1: Option<String>,
    inqr_dvsn_2: Option<String>,
) -> Result<response::stock::order::Body::InquirePsblRvsecncl, Error>
```

> **주의:** 실전투자 전용 API입니다. 모의투자 환경에서도 실전투자 엔드포인트로 요청됩니다.

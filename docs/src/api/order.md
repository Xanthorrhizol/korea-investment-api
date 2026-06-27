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

---

## inquire_psbl_order — 매수가능 조회 `[v1_국내주식-007]`

특정 종목에 대한 매수가능 금액 및 수량을 조회합니다.

```rust
pub async fn inquire_psbl_order(
    &self,
    pdno: &str,
    ord_unpr: Option<String>,
    ord_dvsn: OrderClass,
    cma_evlu_amt_icld_yn: bool,
    ovrs_icld_yn: bool,
) -> Result<response::stock::order::psbl_order::InquirePsblOrder, Error>
```

### 파라미터

| 파라미터 | 타입 | 설명 |
|---|---|---|
| `pdno` | `&str` | 종목 코드 (6자리, 예: `"005930"`) |
| `ord_unpr` | `Option<String>` | 주문 단가. 시장가 조회 시 `None`(공란) |
| `ord_dvsn` | `OrderClass` | 주문 구분 (지정가, 시장가 등) |
| `cma_evlu_amt_icld_yn` | `bool` | CMA 평가금액 포함 여부 (`true` → `"Y"`) |
| `ovrs_icld_yn` | `bool` | 해외 포함 여부 (`true` → `"Y"`) |

> **수량 조회 시:** 정확한 매수가능수량을 보려면 `ord_dvsn`을 `OrderClass::Market`(시장가)으로 지정하세요. 지정가(`OrderClass::Limit`)는 종목증거금율이 반영되지 않습니다.

### 응답

`output`(단건)에서 다음 값을 확인합니다.

| 필드 | 설명 |
|---|---|
| `nrcvb_buy_amt` / `nrcvb_buy_qty` | 미수 없는 매수금액 / 수량 |
| `max_buy_amt` / `max_buy_qty` | 최대 매수금액 / 수량 (미수 사용 시) |
| `ord_psbl_cash` | 주문가능현금 |

### 예시

```rust
use korea_investment_api::types::OrderClass;

// 삼성전자 매수가능 조회 (시장가 기준)
let psbl = api.order.inquire_psbl_order(
    "005930",
    None,                // 시장가 조회 → 단가 공란
    OrderClass::Market,
    false,               // CMA 평가금액 미포함
    false,               // 해외 미포함
).await?;

if let Some(output) = psbl.output() {
    println!("미수없는 매수가능수량: {}", output.nrcvb_buy_qty());
}
```

### 지원 환경

실전 · 모의 모두 지원합니다.

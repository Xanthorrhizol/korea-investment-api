# 타입 레퍼런스

주요 타입들은 `korea_investment_api::types` 모듈에 있습니다.

## Environment

API 환경을 지정합니다.

```rust
pub enum Environment {
    Real,     // 실전투자
    Virtual,  // 모의투자
}
```

## Account

계좌 정보를 담는 구조체입니다.

```rust
pub struct Account {
    pub cano: String,         // 계좌번호 앞 8자리
    pub acnt_prdt_cd: String, // 계좌번호 뒤 2자리
}
```

## Direction

매수/매도 방향을 나타냅니다.

```rust
pub enum Direction {
    Bid, // 매수
    Ask, // 매도
}
```

## OrderClass

주문 유형을 나타냅니다. 주요 값:

| 값 | 설명 |
|---|---|
| `Limit` | 지정가 |
| `Market` | 시장가 |
| `BestLimit` | 최우선 지정가 |
| 기타 | API 포털 참고 |

## CorrectionClass

정정/취소 구분입니다.

```rust
pub enum CorrectionClass {
    Correct, // 정정
    Cancel,  // 취소
}
```

## MarketCode

시장/상품 구분입니다.

```rust
pub enum MarketCode {
    Stock, // 주식
    Etf,   // ETF
}
```

## PeriodCode

조회 기간 코드입니다.

```rust
pub enum PeriodCode {
    ThirtyDays,   // 30일 (일별)
    ThirtyWeeks,  // 30주 (주별)
    ThirtyMonths, // 30개월 (월별)
}
```

## TrId

API 거래 ID입니다. 주요 값:

| 값 | 설명 |
|---|---|
| `RealStockCashBidOrder` | 실전 현금 매수 주문 |
| `RealStockCashAskOrder` | 실전 현금 매도 주문 |
| `VirtualStockCashBidOrder` | 모의 현금 매수 주문 |
| `VirtualStockCashAskOrder` | 모의 현금 매도 주문 |
| `DailyPrice` | 일자별 시세 |
| `VolumeRank` | 거래량 순위 |
| `RealtimeExecKrx` | KRX 실시간 체결 |
| `RealtimeOrdbKrx` | KRX 실시간 호가 |
| `RealtimeExecNxt` | NXT 실시간 체결 |
| `RealtimeOrdbNxt` | NXT 실시간 호가 |
| `RealtimeExecUnion` | 통합 실시간 체결 |
| `RealtimeOrdbUnion` | 통합 실시간 호가 |
| `RealRealtimeMyExec` | 실전 체결통보 |
| `VirtualRealtimeMyExec` | 모의 체결통보 |

## Enum 직렬화 패턴

모든 Enum은 API 전송 코드 문자열로 직렬화됩니다. `Display` 구현이 wire format 코드를 반환하고, `SerializeDisplay`로 serde 직렬화됩니다.

```rust
let tr_id_str: String = TrId::DailyPrice.into(); // "FHKST01010400"
```

## 스트림 타입

실시간 데이터 파싱 타입은 `korea_investment_api::types::stream::stock`에 있습니다:

| 타입 | 설명 |
|---|---|
| `Exec` | 체결 데이터 (H0STCNT0) |
| `Ordb` | 호가 데이터 (H0STASP0) |
| `MyExec` | 체결통보 (H0STCNI0/9) |

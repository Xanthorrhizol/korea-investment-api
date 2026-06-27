# 한국투자증권 API for Rust

이 라이브러리는 [한국투자증권 Open API](https://apiportal.koreainvestment.com)를 Rust에서 사용하기 위한 비공식 클라이언트입니다.

> **주의:** 이 라이브러리는 공식 라이브러리가 아닙니다. 사용 전 [공식 API 포털](https://apiportal.koreainvestment.com)에서 이용약관을 확인하세요.

## 지원 기능

| 카테고리 | 기능 | 지원 환경 |
|---|---|---|
| **인증** | OAuth 토큰 발급/관리 | 실전 · 모의 |
| **인증** | WebSocket 접속키(Approval Key) 발급 | 실전 · 모의 |
| **인증** | HashKey 생성 | 실전 · 모의 |
| **주문** | 현금 매수/매도 | 실전 · 모의 |
| **주문** | 신용 매수/매도 | 실전만 |
| **주문** | 정정/취소 | 실전 · 모의 |
| **주문** | 일별 주문체결 조회 | 실전 · 모의 |
| **주문** | 잔고 조회 | 실전 · 모의 |
| **주문** | 정정취소가능주문 조회 | 실전만 |
| **주문** | 매수가능 조회 | 실전 · 모의 |
| **시세** | 주식 일자별 시세 | 실전 · 모의 |
| **시세** | 거래량 순위 | 실전만 |
| **시세** | 관심종목 그룹 조회 | 실전만 |
| **시세** | 관심종목 그룹별 종목 조회 | 실전만 |
| **시세** | 주식 기본 정보 조회 | 실전만 |
| **실시간** | 체결 스트림 (KRX/NXT/Union) | 실전 · 모의 |
| **실시간** | 호가 스트림 (KRX/NXT/Union) | 실전 · 모의 |
| **실시간** | 체결통보 스트림 | 실전 · 모의 |

## 엔트리 포인트

모든 API는 [`KoreaInvestmentApi`](./getting-started/quick-start.md) 구조체를 통해 접근합니다.

```rust
use korea_investment_api::KoreaInvestmentApi;

let api = KoreaInvestmentApi::new(/* ... */).await?;

api.auth    // 인증
api.order   // 주문
api.quote   // 시세
api.k_data  // 실시간 데이터
```

# 인증 (Auth)

`api.auth`는 OAuth 토큰 관리, WebSocket 접속키 발급, HashKey 생성을 담당합니다.

## 구조체 필드

| 필드 | 타입 | 설명 |
|---|---|---|
| `auth` | `Auth` | 기본(모의/실전) 환경 인증 |
| `real_auth` | `Auth` | 실전투자 전용 API용 인증 |

## 토큰 관리

토큰은 `KoreaInvestmentApi::new()` 생성 시 자동으로 발급됩니다. `token` 파라미터에 기존 값을 전달하면 재사용합니다.

```rust
// 자동 발급
KoreaInvestmentApi::new(..., None, None, ...).await?;

// 캐시된 토큰 재사용
KoreaInvestmentApi::new(..., Some(cached_token), Some(cached_key), ...).await?;
```

### 토큰 조회

```rust
let token: Option<String> = api.auth.get_token();
let approval_key: Option<String> = api.auth.get_approval_key();
```

## HashKey

`POST` 요청의 무결성 검증을 위해 HashKey가 사용됩니다. 주문 API에서 내부적으로 자동 처리됩니다.

```rust
let hash = api.auth.get_hash(json_body_string).await?;
```

## 레이트 리밋

전역 `LAST_CALL` mutex로 호출 빈도를 제한합니다:

| 환경 | 최대 호출 빈도 |
|---|---|
| 실전 (Real) | 초당 20회 |
| 모의 (Virtual) | 초당 2회 |

각 API 호출 전에 자동으로 대기 처리됩니다.

## 환경 엔드포인트

| 환경 | REST | WebSocket |
|---|---|---|
| 실전 | `openapi.koreainvestment.com:9443` | `ops.koreainvestment.com:21000` |
| 모의 | `openapivts.koreainvestment.com:29443` | `ops.koreainvestment.com:31000` |

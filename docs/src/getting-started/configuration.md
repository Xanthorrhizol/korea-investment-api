# 설정 파일

예제 바이너리 및 `export_config`를 사용할 때 `config.toml` 파일 형식을 사용합니다.

## config.toml 형식

```toml
hts_id        = "your-hts-id"
cano          = "12345678"      # 계좌번호 앞 8자리
acnt_prdt_cd  = "01"            # 계좌번호 뒤 2자리
app_key       = "your-app-key"
app_secret    = "your-app-secret"
real_app_key  = "your-real-app-key"       # 실전투자 전용 API 이용 시
real_app_secret = "your-real-app-secret"  # 실전투자 전용 API 이용 시
environment   = "Virtual"       # "Real" 또는 "Virtual"

# 아래 항목은 선택 사항 (캐싱 목적)
approval_key      = ""   # WebSocket 접속키 (캐시)
token             = ""   # OAuth 토큰 (캐시)
real_approval_key = ""   # 실전투자 WebSocket 접속키 (캐시)
real_token        = ""   # 실전투자 OAuth 토큰 (캐시)
```

## 항목 설명

| 항목 | 필수 | 설명 |
|---|---|---|
| `hts_id` | 필수 | HTS 로그인 ID (체결통보 구독에 사용) |
| `cano` | 필수 | 계좌번호 앞 8자리 |
| `acnt_prdt_cd` | 필수 | 계좌번호 뒤 2자리 |
| `app_key` | 필수 | API 포털에서 발급한 App Key |
| `app_secret` | 필수 | API 포털에서 발급한 App Secret |
| `real_app_key` | 조건부 | 실전투자 전용 API 사용 시 필요 |
| `real_app_secret` | 조건부 | 실전투자 전용 API 사용 시 필요 |
| `environment` | 필수 | `"Real"` (실전) 또는 `"Virtual"` (모의) |
| `token` | 선택 | 기존 OAuth 토큰 (캐시). 비어있으면 자동 발급 |
| `approval_key` | 선택 | 기존 WebSocket 접속키 (캐시). 비어있으면 자동 발급 |

## 토큰 캐싱

`KoreaInvestmentApi::export_config`를 호출하면 현재 토큰과 접속키가 `config.toml`에 저장됩니다. 이후 실행 시 기존 토큰을 재사용하여 불필요한 API 호출을 줄입니다.

```rust
// 프로그램 종료 전 토큰 저장
api.export_config(&config)?;
```

> **주의:** `config.toml`에는 민감한 인증 정보가 포함됩니다. `.gitignore`에 추가하여 버전 관리에서 제외하세요.

## 환경변수

| 환경변수 | 기본값 | 설명 |
|---|---|---|
| `CHANNEL_SIZE` | `16384` | WebSocket 데이터 스트림 내부 broadcast 채널 크기. 메시지 유실이 발생할 경우 더 높은 값으로 설정하세요. |

```bash
CHANNEL_SIZE=32768 cargo run
```

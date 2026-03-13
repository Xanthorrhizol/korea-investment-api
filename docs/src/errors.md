# 에러 처리

모든 API 함수는 `Result<T, Error>`를 반환합니다. `Error` 열거형은 `src/lib.rs`에 정의되어 있습니다.

## Error 열거형

```rust
pub enum Error {
    // 외부 라이브러리 에러
    WebSocket(tokio_tungstenite::tungstenite::Error),
    ReqwestError(reqwest::Error),
    Utf8Error(std::str::Utf8Error),
    JsonError(json::JsonError),
    ChronoError(chrono::ParseError),
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    Base64DecodeError(base64::DecodeError),
    UrlParseError(url::ParseError),
    TomlSerializeError(toml::ser::Error),
    IoError(std::io::Error),

    // 커스텀 에러
    AuthInitFailed(&'static str),
    BrokenProtocol(&'static str, String),
    InvalidData,
    WrongTrId(TrId, &'static str),
    AesInvalidLength,
    AesDecryptError(String),
    NotSupported(&'static str),
}
```

## 주요 에러 설명

| 에러 | 발생 상황 |
|---|---|
| `AuthInitFailed` | 토큰 또는 접속키가 초기화되지 않은 상태에서 API 호출 |
| `WrongTrId` | `subscribe_market`에 잘못된 `TrId` 전달 |
| `NotSupported` | 모의투자에서 신용주문 등 미지원 기능 호출 |
| `AesDecryptError` | 체결통보 복호화 실패 |
| `BrokenProtocol` | 서버 응답이 예상과 다를 때 |

## 에러 처리 예시

```rust
use korea_investment_api::Error;

match api.order.order_cash(/* ... */).await {
    Ok(result) => println!("주문 완료: {:?}", result),
    Err(Error::AuthInitFailed(field)) => {
        eprintln!("인증 정보 없음: {}", field);
    }
    Err(Error::ReqwestError(e)) => {
        eprintln!("HTTP 에러: {}", e);
    }
    Err(e) => {
        eprintln!("기타 에러: {}", e);
    }
}
```

## `?` 연산자 사용

`Error`는 `thiserror`로 구현되어 있어 `?` 연산자와 함께 편리하게 사용할 수 있습니다.

```rust
async fn run() -> Result<(), korea_investment_api::Error> {
    let api = KoreaInvestmentApi::new(/* ... */).await?;
    let prices = api.quote.daily_price(/* ... */).await?;
    Ok(())
}
```

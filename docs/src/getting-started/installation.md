# 설치

## 요구사항

- Rust 1.70 이상
- Tokio async runtime

## Cargo.toml 추가

`Cargo.toml`의 `[dependencies]`에 아래를 추가합니다:

```toml
[dependencies]
korea_investment_api = "0.x"  # crates.io 최신 버전 확인
tokio = { version = "1", features = ["full"] }
```

> crates.io에 publish된 최신 버전은 [crates.io/crates/korea_investment_api](https://crates.io/crates/korea_investment_api)에서 확인하세요.

## 예제 바이너리 빌드

이 저장소를 직접 클론해서 예제 바이너리를 빌드할 수 있습니다:

```bash
git clone https://github.com/systeminit/korea-investment-api
cd korea-investment-api

# 예제 바이너리 빌드 (structopt feature 필요)
cargo build --features example --bin example

# 실행 (config.toml 경로를 인자로 전달)
cargo run --features example --bin example -- config.toml
```

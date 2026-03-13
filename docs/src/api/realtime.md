# 실시간 데이터 (Realtime)

`api.k_data`는 WebSocket 기반의 실시간 시세 데이터를 제공합니다. `src/stock/data.rs`에 구현되어 있습니다.

## 스트림 종류

| TrId | 설명 | 데이터 타입 |
|---|---|---|
| `RealtimeExecKrx` | KRX 체결 | `Exec` |
| `RealtimeExecNxt` | NXT 체결 | `Exec` |
| `RealtimeExecUnion` | 통합 체결 | `Exec` |
| `RealtimeOrdbKrx` | KRX 호가 | `Ordb` |
| `RealtimeOrdbNxt` | NXT 호가 | `Ordb` |
| `RealtimeOrdbUnion` | 통합 호가 | `Ordb` |
| `RealRealtimeMyExec` / `VirtualRealtimeMyExec` | 내 체결통보 | `MyExec` |

---

## subscribe_market — 시세 구독

종목 실시간 체결 또는 호가 데이터를 구독합니다.

```rust
pub async fn subscribe_market<T: StreamParser<R> + Send, R: Clone + Send>(
    &mut self,
    tr_key: &str,
    tr_id: TrId,
) -> Result<(Option<UnboundedReceiver<T>>, SubscribeResponse), Error>
```

### 파라미터

| 파라미터 | 타입 | 설명 |
|---|---|---|
| `tr_key` | `&str` | 종목 코드 (6자리) |
| `tr_id` | `TrId` | 스트림 종류 (`RealtimeExecKrx` 등) |

### 반환값

- `Option<UnboundedReceiver<T>>`: 새로 생성된 수신 채널. 이미 구독 중이면 `None`
- `SubscribeResponse`: 구독 응답 (성공 여부, 메시지)

### 예시 — 체결 구독

```rust
use korea_investment_api::types::{TrId};
use korea_investment_api::types::stream::stock::{Exec, ExecRow};

let (rx, response) = api.k_data.subscribe_market::<Exec, ExecRow>(
    "005930",
    TrId::RealtimeExecKrx,
).await?;

assert!(response.is_success());

if let Some(mut rx) = rx {
    while let Some(exec) = rx.recv().await {
        println!("체결가: {:?}", exec);
    }
}
```

### 예시 — 호가 구독

```rust
use korea_investment_api::types::stream::stock::{Ordb, OrdbRow};

let (rx, _) = api.k_data.subscribe_market::<Ordb, OrdbRow>(
    "005930",
    TrId::RealtimeOrdbKrx,
).await?;
```

### PINGPONG 처리

서버에서 `PINGPONG` 프레임이 오면 자동으로 에코 응답합니다. 사용자 코드에서 처리할 필요가 없습니다.

---

## subscribe_my_exec — 체결통보 구독

내 계좌의 주문 체결 통보를 실시간으로 수신합니다.

```rust
pub async fn subscribe_my_exec(
    &mut self,
) -> Result<(UnboundedReceiver<MyExec>, SubscribeResponse), Error>
```

### 보안

체결통보 스트림 데이터는 **AES-256-CBC**로 암호화되어 전송됩니다. 구독 응답에 포함된 `iv`와 `key`를 사용하여 내부적으로 자동 복호화합니다.

### 예시

```rust
use korea_investment_api::types::stream::stock::MyExec;

let (mut rx, response) = api.k_data.subscribe_my_exec().await?;

assert!(response.is_success());

while let Some(my_exec) = rx.recv().await {
    println!("체결통보: {:?}", my_exec);
}
```

---

## unsubscribe_market — 시세 구독 해제

구독 중인 종목 시세 스트림을 해제합니다.

```rust
pub async fn unsubscribe_market(
    &mut self,
    tr_key: &str,
    tr_id: TrId,
) -> Result<SubscribeResponse, Error>
```

### 예시

```rust
let response = api.k_data.unsubscribe_market(
    "005930",
    TrId::RealtimeExecKrx,
).await?;
```

---

## 자동 구독 해제

`KoreaStockData`가 Drop될 때, 진행 중인 모든 구독이 자동으로 해제됩니다.

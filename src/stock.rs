use crate::{Account, Error};
use websocket::native_tls::{TlsConnector, TlsStream};

pub struct Korea {
    client: reqwest::Client,
    wsclient: websocket::client::sync::Client<TlsStream<std::net::TcpStream>>,
    approval_key: String,
    hash: String,
    account: Account,
}

impl Korea {
    /// 국내 주식 주문/시세에 관한 API
    /// [국내주식주문](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_aade4c72-5fb7-418a-9ff2-254b4d5f0ceb)
    /// [국내주식시세](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock-quotations#L_07802512-4f49-4486-91b4-1050b6f5dc9d)
    pub fn new(
        client: &reqwest::Client,
        endpoint_url: &str,
        approval_key: String,
        hash: String,
        account: Account,
    ) -> Result<Self, Error> {
        let wsclient = websocket::ClientBuilder::new(endpoint_url)?
            .connect_secure(Some(TlsConnector::new()?))?;
        Ok(Self {
            client: client.clone(),
            wsclient,
            approval_key,
            hash,
            account,
        })
    }

    /// 주식주문(현금)[v1_국내주식-001]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_aade4c72-5fb7-418a-9ff2-254b4d5f0ceb)
    pub fn order_cash() -> Result {}
    // TODO: 주식주문(신용)
    // 주식주문(정정취소)
    // 주식정정취소가능주문조회
    // 주식일별주문체결조회
}

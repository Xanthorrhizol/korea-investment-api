use crate::Error;
use websocket::native_tls::{TlsConnector, TlsStream};

pub struct Korea {
    client: websocket::client::sync::Client<TlsStream<std::net::TcpStream>>,
    appkey: String,
    appsecret: String,
    token: String,
    hash: String,
}

impl Korea {
    /// 국내 주식 주문/시세에 관한 API
    /// [국내주식주문](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_aade4c72-5fb7-418a-9ff2-254b4d5f0ceb)
    /// [국내주식시세](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock-quotations#L_07802512-4f49-4486-91b4-1050b6f5dc9d)
    pub fn new(
        endpoint_url: String,
        appkey: String,
        appsecret: String,
        token: String,
        hash: String,
    ) -> Result<Self, Error> {
        let client = websocket::ClientBuilder::new(&endpoint_url)?
            .connect_secure(Some(TlsConnector::new()?))?;
        Ok(Self {
            client,
            appkey,
            appsecret,
            token,
            hash,
        })
    }
}

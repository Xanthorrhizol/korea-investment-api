pub struct Korea {
    client: reqwest::Client,
    appkey: String,
    appsecret: String,
    token: String,
    hash: String,
}

impl Korea {
    /// 국내 주식 주문/시세에 관한 API
    /// [국내주식주문](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock#L_aade4c72-5fb7-418a-9ff2-254b4d5f0ceb)
    /// [국내주식시세](https://apiportal.koreainvestment.com/apiservice/apiservice-domestic-stock-quotations#L_07802512-4f49-4486-91b4-1050b6f5dc9d)
    /// 아.... 실시간 시세는 웹소켓이라네
    /// TODO: 웹소켓 고고
    pub async fn new(
        client: reqwest::Client,
        appkey: String,
        appsecret: String,
        token: String,
        hash: String,
    ) -> Self {
        Self {
            client,
            appkey,
            appsecret,
            token,
            hash,
        }
    }
}

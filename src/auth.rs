use crate::types;
use reqwest::header::{HeaderMap, HeaderValue};

/// Auth
/// [OAuth 인증 관련](https://apiportal.koreainvestment.com/apiservice/oauth2#L_214b9e22-8f2e-4fba-9688-587279f1061a)
pub struct Auth {
    client: reqwest::Client,
    endpoint_url: String,
    appkey: String,
    appsecret: String,
    hash: Option<String>,
    token: Option<String>,
}

impl Auth {
    /// OAuth API 사용을 위한 구조체 생성
    /// hash, token은 None으로 시작
    /// create_hash, create_token 함수를 호출해야 hash, token 값이 저장됨
    pub fn new(
        client: &reqwest::Client,
        endpoint_url: &str,
        appkey: String,
        appsecret: String,
    ) -> Self {
        Self {
            client: client.clone(),
            endpoint_url: endpoint_url.to_string(),
            appkey,
            appsecret,
            hash: None,
            token: None,
        }
    }

    /// 구조체에 저장되어 있는 hash를 반환
    pub fn get_hash(&self) -> String {
        self.hash.unwrap().clone()
    }

    /// 구조체에 저장되어 있는 token을 반환
    pub fn get_token(&self) -> String {
        self.token.unwrap().clone()
    }

    /// Hashkey
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/oauth2#L_214b9e22-8f2e-4fba-9688-587279f1061a)
    /// hash값을 얻어와서 반환함과 동시에 구조체의 hash를 업데이트
    pub async fn create_hash(&mut self) -> String {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert("appkey", HeaderValue::from_str(&self.appkey).unwrap());
        headers.insert("appsecret", HeaderValue::from_str(&self.appsecret).unwrap());
        let hash = self
            .client
            .post(format!("{}/uapi/hashkey", self.endpoint_url))
            .headers(headers)
            .body("{}")
            .send()
            .await
            .unwrap()
            .json::<types::HashKeyResponse>()
            .await
            .unwrap()
            .get_hash();
        self.hash = Some(hash.clone());
        hash
    }

    /// 접근토큰발급(P)[인증-001]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/oauth2#L_fa778c98-f68d-451e-8fff-b1c6bfe5cd30)
    /// token값을 얻어와서 반환함과 동시에 구조체의 token을 업데이트
    pub async fn create_token(&mut self) -> String {
        let token = self
            .client
            .post(format!("{}/uapi/tokenP", self.endpoint_url))
            .header("Content-Type", "application/json")
            .body(format!(
                "{{\"appkey\": \"{}\",\"appsecret\":\"{}\"}}",
                self.appkey, self.appsecret
            ))
            .send()
            .await
            .unwrap()
            .json::<types::TokenCreationResponse>()
            .await
            .unwrap()
            .get_access_token();
        self.token = Some(token.clone());
        token
    }

    /// 접근토큰폐기(P)[인증-002]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice/oauth2#L_dd3cb447-5034-4711-8c88-62c913429c7b)
    /// token을 폐기. 폐기 결과 리스폰스를 그대로 반환
    /// 리스폰스 형식: {
    ///     code: u32,
    ///     message: String,
    /// }
    pub async fn revoke_token(&self) -> types::TokenRevokeResponse {
        self.client
            .post(format!("{}/uapi/revokeP", &self.endpoint_url))
            .header("Content-Type", "application/json")
            .body(format!(
                "{{\"appkey\":\"{}\",\"appsecret\":\"{}\",\"token\":\"{}\"}}",
                self.appkey,
                self.appsecret,
                self.token.clone().unwrap()
            ))
            .send()
            .await
            .unwrap()
            .json::<types::TokenRevokeResponse>()
            .await
            .unwrap()
    }
}

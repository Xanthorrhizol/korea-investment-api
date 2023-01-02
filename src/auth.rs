use crate::types;
use reqwest::header::{HeaderMap, HeaderValue};

pub struct Auth {
    client: reqwest::Client,
    endpoint_url: String,
    appkey: String,
    appsecret: String,
    pub hash: String,
    pub token: String,
}

impl Auth {
    pub async fn new(
        client: &reqwest::Client,
        endpoint_url: &str,
        appkey: String,
        appsecret: String,
    ) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert("appkey", HeaderValue::from_str(&appkey).unwrap());
        headers.insert("appsecret", HeaderValue::from_str(&appsecret).unwrap());
        let hash = client
            .post(format!("{}/uapi/hashkey", &endpoint_url))
            .headers(headers)
            .body("{}")
            .send()
            .await
            .unwrap()
            .json::<types::HashKeyResponse>()
            .await
            .unwrap()
            .get_hash();
        let token = client
            .post(format!("{}/uapi/tokenP", &endpoint_url))
            .header("Content-Type", "application/json")
            .body(format!(
                "{{\"appkey\": \"{}\",\"appsecret\":\"{}\"}}",
                appkey, appsecret
            ))
            .send()
            .await
            .unwrap()
            .json::<types::TokenCreationResponse>()
            .await
            .unwrap()
            .get_access_token();
        Self {
            client: client.clone(),
            endpoint_url: endpoint_url.to_string(),
            appkey,
            appsecret,
            hash,
            token,
        }
    }

    pub async fn revoke_token(&self) -> types::TokenRevokeResponse {
        self.client
            .post(format!("{}/uapi/revokeP", &self.endpoint_url))
            .header("Content-Type", "application/json")
            .body(format!(
                "{{\"appkey\":\"{}\",\"appsecret\":\"{}\",\"token\":\"{}\"}}",
                self.appkey, self.appsecret, self.token
            ))
            .send()
            .await
            .unwrap()
            .json::<types::TokenRevokeResponse>()
            .await
            .unwrap()
    }
}

mod types;

use reqwest::header::{HeaderMap, HeaderValue};

const REAL_ENDPOINT_URL: String = "https://openapi.koreainvestment.com:9443".to_string();
const VIRTUAL_ENDPOINT_URL: String = "https://openapivts.koreainvestment.com:29443".to_string();
const HASHKEY_PATH: String = "/uapi/hashkey".to_string();

/// 사용할 계좌
/// 실전투자: Real
/// 모의투자: Virtual
pub enum Environment {
    Real,
    Virtual,
}

pub struct KoreaInvestmentApi {
    client: reqwest::Client,
    endpoint_url: String,
    appkey: String,
    appsecret: String,
    hash: String,
    token: String,
}

impl KoreaInvestmentApi {
    pub async fn new(env: Environment, appkey: String, appsecret: String) -> Self {
        let endpoint_url = match env {
            Environment::Real => REAL_ENDPOINT_URL,
            Environment::Virtual => VIRTUAL_ENDPOINT_URL,
        };
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert("appkey", HeaderValue::from_str(&appkey).unwrap());
        let hash = client
            .post(format!("{}{}", endpoint_url, HASHKEY_PATH))
            .headers(headers)
            .body("{}")
            .send()
            .await
            .unwrap()
            .json::<types::HashKeyResponse>()
            .await
            .unwrap()
            .get_hash();

        Self {
            client,
            endpoint_url,
            appkey,
            appsecret,
            hash,
            token,
        }
    }
}

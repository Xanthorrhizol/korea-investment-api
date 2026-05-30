use crate::Error;
use crate::types::{Environment, request, response};
use reqwest::header::{HeaderMap, HeaderValue};
use std::sync::{Arc, RwLock};

/// Auth
/// [OAuth 인증 관련](https://apiportal.koreainvestment.com/apiservice-apiservice?/oauth2/tokenP)
#[derive(Clone, Debug)]
pub struct Auth {
    client: reqwest::Client,
    environment: Environment,
    endpoint_url: String,
    appkey: String,
    appsecret: String,
    // Shared so a runtime refresh (create_token) propagates to every clone
    // held by the sub-clients (order/quote/data).
    token: Arc<RwLock<Option<String>>>,
    approval_key: Arc<RwLock<Option<String>>>,
}

impl Auth {
    /// OAuth API 사용을 위한 구조체 생성
    /// hash, token, approval_key는 None으로 시작
    /// create_hash, create_token, create_approval_key 함수를 호출해야 hash, token 값이 저장됨
    pub fn new(
        client: &reqwest::Client,
        environment: Environment,
        appkey: &str,
        appsecret: &str,
    ) -> Self {
        let endpoint_url = match environment {
            Environment::Real => "https://openapi.koreainvestment.com:9443",
            Environment::Virtual => "https://openapivts.koreainvestment.com:29443",
        }
        .to_string();
        Self {
            client: client.clone(),
            environment,
            endpoint_url,
            appkey: appkey.to_string(),
            appsecret: appsecret.to_string(),
            token: Arc::new(RwLock::new(None)),
            approval_key: Arc::new(RwLock::new(None)),
        }
    }

    /// 구조체에 저장되어 있는 token을 반환
    pub fn get_token(&self) -> Option<String> {
        self.token.read().unwrap().clone()
    }

    /// 구조체에 저장되어 있는 approval_key 반환
    pub fn get_approval_key(&self) -> Option<String> {
        self.approval_key.read().unwrap().clone()
    }

    /// 구조체에 저장되어 있는 appkey 반환
    pub fn get_appkey(&self) -> String {
        self.appkey.clone()
    }

    /// 구조체에 저장되어 있는 appsecret 반환
    pub fn get_appsecret(&self) -> String {
        self.appsecret.clone()
    }

    /// 실시간 (웹소켓) 접속키 발급[실시간-000]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/oauth2/Approval)
    /// 웹소켓 접속키를 발급받아서 반환함과 동시에 구조체의 approval_key 업데이트
    pub async fn create_approval_key(&mut self) -> Result<String, Error> {
        crate::wait(self.environment).await;
        let approval_key = self
            .client
            .post(format!("{}/oauth2/Approval", self.endpoint_url))
            .header("Content-Type", "application/json")
            .body(
                serde_json::json!(request::auth::ApprovalKeyCreationBody::new(
                    self.appsecret.clone(),
                    self.appkey.clone(),
                ))
                .to_string(),
            )
            .send()
            .await?
            .json::<response::auth::Body::ApprovalKeyCreation>()
            .await?
            .get_approval_key();
        crate::update_last_call();
        *self.approval_key.write().unwrap() = Some(approval_key.clone());
        Ok(approval_key)
    }

    pub fn set_approval_key(&mut self, approval_key: String) {
        *self.approval_key.write().unwrap() = Some(approval_key);
    }

    /// Hashkey
    /// [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/hashkey)
    /// hash값을 얻어와서 반환
    pub async fn get_hash(&self, json: String) -> Result<String, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert("appkey", HeaderValue::from_str(&self.appkey).unwrap());
        headers.insert("appsecret", HeaderValue::from_str(&self.appsecret).unwrap());
        crate::wait(self.environment).await;
        let hash = self
            .client
            .post(format!("{}/uapi/hashkey", self.endpoint_url))
            .headers(headers)
            .body(json)
            .send()
            .await?
            .json::<response::auth::Body::HashKey>()
            .await?
            .get_hash();
        crate::update_last_call();
        Ok(hash)
    }

    /// 접근토큰발급(P)[인증-001]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/oauth2/tokenP)
    /// token값을 얻어와서 반환함과 동시에 구조체의 token을 업데이트
    pub async fn create_token(&mut self) -> Result<String, Error> {
        crate::wait(self.environment).await;
        let token = self
            .client
            .post(format!("{}/oauth2/tokenP", self.endpoint_url))
            .header("Content-Type", "application/json")
            .body(
                serde_json::json!(request::auth::TokenCreationBody::new(
                    self.appsecret.clone(),
                    self.appkey.clone(),
                ))
                .to_string(),
            )
            .send()
            .await?
            .json::<response::auth::Body::TokenCreation>()
            .await?
            .get_access_token();
        crate::update_last_call();
        *self.token.write().unwrap() = Some(token.clone());
        Ok(token)
    }

    pub fn set_token(&mut self, token: String) {
        *self.token.write().unwrap() = Some(token);
    }

    /// 접근토큰폐기(P)[인증-002]
    /// [Docs](https://apiportal.koreainvestment.com/apiservice-apiservice?/oauth2/revokeP)
    /// token을 폐기. 폐기 결과 리스폰스를 그대로 반환
    /// 리스폰스 형식: {
    ///     code: u32,
    ///     message: String,
    /// }
    pub async fn revoke_token(&self) -> Result<response::auth::Body::TokenRevoke, Error> {
        crate::wait(self.environment).await;
        let result = self
            .client
            .post(format!("{}/oauth2/revokeP", &self.endpoint_url))
            .header("Content-Type", "application/json")
            .body(
                serde_json::json!(request::auth::TokenRevokeBody::new(
                    self.appkey.clone(),
                    self.appsecret.clone(),
                    match self.get_token() {
                        Some(token) => token,
                        None => {
                            return Err(Error::AuthInitFailed("token"));
                        }
                    }
                ))
                .to_string(),
            )
            .send()
            .await?
            .json::<response::auth::Body::TokenRevoke>()
            .await?;
        crate::update_last_call();
        Ok(result)
    }
}

#![allow(non_snake_case)]

pub mod Body {
    use serde::{Deserialize, Serialize};

    /// 실시간 (웹소켓) 접속키 발급
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ApprovalKeyCreation {
        approval_key: String,
    }
    impl ApprovalKeyCreation {
        pub fn get_approval_key(&self) -> String {
            self.approval_key.clone()
        }
    }

    /// Hashkey
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct HashKey {
        HASH: String,
    }
    impl HashKey {
        pub fn get_hash(&self) -> String {
            self.HASH.clone()
        }
    }

    /// 접근토큰발급(P)
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TokenCreation {
        access_token: String,
        token_type: String,
        expires_in: u32,
    }
    impl TokenCreation {
        pub fn get_access_token(&self) -> String {
            self.access_token.clone()
        }
    }

    /// 접근토큰폐기(P)
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TokenRevoke {
        pub code: u32,
        pub message: String,
    }
}

#![allow(non_snake_case)]
pub mod Header {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Stock {
        tr_id: String,   // 거래ID(요청한 tr_id)
        tr_cont: String, // 연속 거래 여부(F or M: 다음 데이터 있음 / D or E: 마지막 데이터)
        gt_uid: String,  // Global UID(거래고유번호)
    }
}

pub mod Body {
    use super::Output;
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
        JsonBody: String,
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

    /// 주식주문(현금, 신용, 정정취소)
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Order {
        tr_cd: String,         // 0: 성공, 0 이외의 값: 실패
        msg_cd: String,        // 응답코드
        msg1: String,          // 응답메시지
        output: Output::Order, // 응답 상세
    }

    /// 주식정정취소가능주문조회
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct InquirePsblRvsecncl {
        tr_cd: String,                       // 0: 성공, 0 이외의 값: 실패
        msg_cd: String,                      // 응답코드
        msg1: String,                        // 응답메시지
        ctx_area_fk100: Option<String>,      // 연속조회검색조건100
        ctx_area_nk100: Option<String>,      // 연속조회키100
        output: Output::InquirePsblRvsecncl, // 응답 상세
    }
}

pub mod Output {
    use serde::{Deserialize, Serialize};

    /// 주식주문(현금, 신용, 정정취소)
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Order {
        KRX_FWDG_ORD_ORGNO: String, // 주문시 한국투자증권 시스템에서 지정된 영업점코드
        ODNO: String,               // 주문번호(주문시 한국투자증권 시스템에서 채번된 주문번호)
        ORD_TMD: String,            // 주문시각(시분초HHMMSS)
    }

    /// 주식정정취소가능주문조회
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct InquirePsblRvsecncl {
        ord_gno_brno: String, // 주문채번지점번호(주문시 한국투자증권 시스템에서 지정된 영업점코드)
        odno: String,         // 주문번호(주문시 한국투자증권 시스템에서 채번된 주문번호)
        orgn_odno: String,    // 원주문번호(정정/취소주문인 경우 원주문번호)
        ord_dvsn_name: String, // 주문구분명
        pdno: String,         // 상품번호(종목번호(뒤 6자리만 해당))
        prdt_name: String,    // 상품명(종목명)
        rvse_cncl_dvsn_name: String, // 정정취소구분명(정정 또는 취소 여부 표시)
        ord_qty: String,      // 주문수량
        ord_unpr: String,     // 주문단가(1주당 주문가격)
        ord_tmd: String,      // 주문시각(시분초HHMMSS)
        tot_ccld_qty: String, // 총체결수량(주문 수량 중 체결된 수량)
        tot_ccld_amt: String, // 총체결금액(주문금액 중 체결금액)
        psbl_qty: String,     // 가능수량(정정/취소 주문 가능 수량)
        sll_buy_dvsn_cd: String, // 매도매수구분코드(01: 매도, 02: 매수)
        ord_dvsn_cd: String,  // 주문구분코드(
        //                         00: 지정가,
        //                         01: 시장가,
        //                         02: 조건부지정가,
        //                         03: 최유리지정가,
        //                         04: 최우선지정가,
        //                         05: 장전 시간외,
        //                         06: 장후 시간외,
        //                         07: 시간외 단일가,
        //                         08: 자기주식,
        //                         09: 자기주식S-Option,
        //                         10: 자기주식금전신탁,
        //                         11: IOC지정가(즉시체결, 잔량취소)
        //                         12: FOK지정가(즉시체결, 잔량취소)
        //                         13: IOC시장가(즉시체결, 잔량취소)
        //                         14: FOK시장가(즉시체결, 잔량취소)
        //                         15: IOC최유리(즉시체결, 잔량취소)
        //                         16: FOK최유리(즉시체결, 잔량취소)
        //                         51: 장중대량
        //                      )
        mgco_aptm_odno: String, // 운용사지정주문번호(주문번호(운용사 통한 주문))
    }
}

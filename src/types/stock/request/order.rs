use crate::types::{CustomerType, TrId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Header {
    authorization: String,
    appkey: String,
    appsecret: String,
    // personalseckey: String // TODO: 법인용
    tr_id: TrId,
    custtype: CustomerType,
}

impl Header {
    pub fn new(token: String, appkey: String, appsecret: String, tr_id: TrId) -> Self {
        Self {
            authorization: token,
            appkey,
            appsecret,
            tr_id,
            custtype: CustomerType::Personal,
        }
    }
}

pub mod Body {
    use crate::types::{
        stock::{CorrectionClass, OrderClass},
        Price, Quantity,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub struct Order {
        cano: String,         // 종합계좌번호(계좌번호 체계(8-2)의 앞 8자리)
        acnt_prdt_cd: String, // 계좌상품코드(계좌번호 체계(8-2)의 뒤 2자리)
        pdno: String,         // 종목코드(6자리)
        ord_dvsn: OrderClass, // 주문구분
        ord_qty: Quantity,    // 주문수량(주문주식수)
        ord_unpr: Price,      // 주문단가(1주당 가격; 시장가는 0으로)
    }

    impl Order {
        pub fn new(
            cano: String,
            acnt_prdt_cd: String,
            pdno: String,
            ord_dvsn: OrderClass,
            ord_qty: Quantity,
            ord_unpr: Price,
        ) -> Self {
            Self {
                cano,
                acnt_prdt_cd,
                pdno,
                ord_dvsn,
                ord_qty,
                ord_unpr,
            }
        }
        pub fn get_json_string(self) -> String {
            serde_json::json!(self).to_string()
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub struct Correction {
        cano: String,               // 종합계좌번호(계좌번호 체계(8-2)의 앞 8자리)
        acnt_prdt_cd: String,       // 계좌상품코드(계좌번호 체계(8-2)의 뒤 2자리)
        krx_fwdg_ord_orgno: String, // 한국거래소전송주문조직번호(주문시 한국투자증권 시스템에서
        // 지정된 영업점코드)
        orgn_odno: String, // 원주문번호(주식일별주문체결조회 API output1의 odno(주문번호) 값 입력.
        // 주문시 한국투자증권 시스템에서 채번된 주문번호)
        ord_dvsn: OrderClass,               // 주문구분
        rvse_cncl_dvsn_cd: CorrectionClass, // 정정취소구분코드
        ord_qty: Quantity,                  // 주문수량(주문주식수)
        ord_unpr: Price,                    // 주문단가([정정] 정정주문 1주당 가격, [취소] "0")
        qty_all_ord_yn: bool, // 잔량전부주문여부([정정/취소] Y: 잔량전부, N: 잔량일부)
    }
    impl Correction {
        pub fn new(
            cano: String,
            acnt_prdt_cd: String,
            krx_fwdg_ord_orgno: String,
            orgn_odno: String,
            ord_dvsn: OrderClass,
            rvse_cncl_dvsn_cd: CorrectionClass,
            ord_qty: Quantity,
            ord_unpr: Price,
            qty_all_ord_yn: bool,
        ) -> Self {
            Self {
                cano,
                acnt_prdt_cd,
                krx_fwdg_ord_orgno,
                orgn_odno,
                ord_dvsn,
                rvse_cncl_dvsn_cd,
                ord_qty,
                ord_unpr,
                qty_all_ord_yn,
            }
        }
        pub fn get_json_string(self) -> String {
            serde_json::json!(self).to_string()
        }
    }
}

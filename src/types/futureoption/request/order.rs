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
        futureoption::{KrxOrderClass, OrderClass, OrderTypeClass},
        Price, Quantity,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub struct Order {
        ord_prcs_cvsn_cd: String,        // 주문처리구분코드(02: 주문전송)
        cano: String,                    // 종합계좌번호(계좌번호 체계(8-2)의 앞 8자리)
        acnt_prdt_cd: String,            // 계좌상품코드(계좌번호 체계(8-2)의 뒤 2자리)
        sll_buy_dvsn_cd: String,         // 종목코드(선물: 6자리, 옵션: 9자리)
        shtn_pdno: String,               // 종목코드(6자리)
        ord_qty: Quantity,               // 주문수량(주문주식수)
        unit_price: Price,               // 주문단가(1주당 가격; 시장가, 최유리지정가는 0으로)
        nmpr_type_cd: OrderTypeClass,    // 호가유형코드
        krx_nmpr_cdnt_cd: KrxOrderClass, // 한국거래소호가조건코드
        ord_dvsn_cd: OrderClass,         // 주문구분코드
    }

    impl Order {
        pub fn new(
            cano: String,
            acnt_prdt_cd: String,
            sll_buy_dvsn_cd: String,
            shtn_pdno: String,
            ord_qty: Quantity,
            unit_price: Price,
            nmpr_type_cd: OrderTypeClass,
            krx_nmpr_cdnt_cd: KrxOrderClass,
            ord_dvsn_cd: OrderClass,
        ) -> Self {
            Self {
                ord_prcs_cvsn_cd: "02".to_string(),
                cano,
                acnt_prdt_cd,
                sll_buy_dvsn_cd,
                shtn_pdno,
                ord_qty,
                unit_price,
                nmpr_type_cd,
                krx_nmpr_cdnt_cd,
                ord_dvsn_cd,
            }
        }
        pub fn get_json_string(self) -> String {
            serde_json::json!(self).to_string()
        }
    }
}

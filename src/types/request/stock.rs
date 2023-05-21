#![allow(non_snake_case)]

pub mod Header {
    use crate::types::{CustomerType, TrId};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Order {
        authorization: String,
        appkey: String,
        appsecret: String,
        // personalseckey: String // TODO: 법인용
        tr_id: TrId,
        custtype: CustomerType,
    }

    impl Order {
        fn new(token: String, appkey: String, appsecret: String, tr_id: TrId) -> Self {
            Self {
                authorization: token,
                appkey,
                appsecret,
                tr_id,
                custtype: CustomerType::Personal,
            }
        }
    }
}

pub mod Body {
    use crate::types::{OrderDivision, Price, Quantity};
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Order {
        CANO: String,            // 종합계좌번호(계좌번호 체계(8-2)의 앞 8자리)
        ACNT_PRDT_CD: String,    // 계좌상품코드(계좌번호 체계(8-2)의 뒤 2자리)
        PDNO: String,            // 종목코드(6자리)
        ORD_DVSN: OrderDivision, // 주문구분
        ORD_QTY: Quantity,       // 주문수량(주문주식수)
        ORD_UNPR: Price,         // 주문단가(1주당 가격; 시장가는 0으로)
    }

    impl Order {
        pub fn new(
            CANO: String,
            ACNT_PRDT_CD: String,
            PDNO: String,
            ORD_DVSN: OrderDivision,
            ORD_QTY: Quantity,
            ORD_UNPR: Price,
        ) -> Self {
            Self {
                CANO,
                ACNT_PRDT_CD,
                PDNO,
                ORD_DVSN,
                ORD_QTY,
                ORD_UNPR,
            }
        }
        pub fn get_json_string(self) -> String {
            format!(
            "{{\"CANO\":\"{}\",\"ACNT_PRDT_CD\":\"{}\",\"PDNO\":\"{}\",\"ORD_DVSN\":\"{}\",\"ORD_QTY\":\"{}\", \"ORD_UNPR\":\"{}\"}}", self.CANO, self.ACNT_PRDT_CD, self.PDNO, Into::<String>::into(self.ORD_DVSN), Into::<String>::into(self.ORD_QTY), Into::<String>::into(self.ORD_UNPR))
        }
    }
}

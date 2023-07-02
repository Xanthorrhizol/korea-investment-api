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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Correction {
        authorization: String,
        appkey: String,
        appsecret: String,
        // personalseckey: String // TODO: 법인용
        tr_id: TrId,
        custtype: CustomerType,
    }

    impl Correction {
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
    use crate::types::{CorrectionClass, OrderClass, Price, Quantity};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Order {
        CANO: String,         // 종합계좌번호(계좌번호 체계(8-2)의 앞 8자리)
        ACNT_PRDT_CD: String, // 계좌상품코드(계좌번호 체계(8-2)의 뒤 2자리)
        PDNO: String,         // 종목코드(6자리)
        ORD_DVSN: OrderClass, // 주문구분
        ORD_QTY: Quantity,    // 주문수량(주문주식수)
        ORD_UNPR: Price,      // 주문단가(1주당 가격; 시장가는 0으로)
    }

    impl Order {
        pub fn new(
            CANO: String,
            ACNT_PRDT_CD: String,
            PDNO: String,
            ORD_DVSN: OrderClass,
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Correction {
        CANO: String,               // 종합계좌번호(계좌번호 체계(8-2)의 앞 8자리)
        ACNT_PRDT_CD: String,       // 계좌상품코드(계좌번호 체계(8-2)의 뒤 2자리)
        KRX_FWDG_ORD_ORGNO: String, // 한국거래소전송주문조직번호(주문시 한국투자증권 시스템에서
        // 지정된 영업점코드)
        ORGN_ODNO: String, // 원주문번호(주식일별주문체결조회 API output1의 odno(주문번호) 값 입력.
        // 주문시 한국투자증권 시스템에서 채번된 주문번호)
        ORD_DVSN: OrderClass,               // 주문구분
        RVSE_CNCL_DVSN_CD: CorrectionClass, // 정정취소구분코드
        ORD_QTY: Quantity,                  // 주문수량(주문주식수)
        ORD_UNPR: Price,                    // 주문단가([정정] 정정주문 1주당 가격, [취소] "0")
        QTY_ALL_ORD_YN: bool, // 잔량전부주문여부([정정/취소] Y: 잔량전부, N: 잔량일부)
    }
    impl Correction {
        pub fn new(
            CANO: String,
            ACNT_PRDT_CD: String,
            KRX_FWDG_ORD_ORGNO: String,
            ORGN_ODNO: String,
            ORD_DVSN: OrderClass,
            RVSE_CNCL_DVSN_CD: CorrectionClass,
            ORD_QTY: Quantity,
            ORD_UNPR: Price,
            QTY_ALL_ORD_YN: bool,
        ) -> Self {
            Self {
                CANO,
                ACNT_PRDT_CD,
                KRX_FWDG_ORD_ORGNO,
                ORGN_ODNO,
                ORD_DVSN,
                RVSE_CNCL_DVSN_CD,
                ORD_QTY,
                ORD_UNPR,
                QTY_ALL_ORD_YN,
            }
        }
        pub fn get_json_string(self) -> String {
            format!(
            "{{\"CANO\":\"{}\",\"ACNT_PRDT_CD\":\"{}\",\"KRX_FWDG_ORD_ORGNO\": \"{}\",\"ORGN_ODNO\":\"{}\",\"ORD_DVSN\":\"{}\",\"RVSE_CNCL_DVSN_CD\":\"{}\",\"ORD_QTY\":\"{}\",\"ORD_UNPR\":\"{}\",\"QTY_ALL_ORD_YN\":\"{}\"}}", self.CANO, self.ACNT_PRDT_CD, self.KRX_FWDG_ORD_ORGNO, self.ORGN_ODNO, Into::<String>::into(self.ORD_DVSN), Into::<String>::into(self.RVSE_CNCL_DVSN_CD), Into::<String>::into(self.ORD_QTY), Into::<String>::into(self.ORD_UNPR), if self.QTY_ALL_ORD_YN { "Y" } else { "N" }.to_string())
        }
    }
}

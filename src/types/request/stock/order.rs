use crate::types::{CustomerType, TrId};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Getters, Setters, Serialize, Deserialize)]
pub struct Header {
    #[getset(get = "pub", set = "pub")]
    authorization: String,
    #[getset(get = "pub", set = "pub")]
    appkey: String,
    #[getset(get = "pub", set = "pub")]
    appsecret: String,
    //#[getset(get = "pub", set = "pub")]
    // personalseckey: String // TODO: 법인용
    #[getset(get = "pub", set = "pub")]
    tr_id: TrId,
    #[getset(get = "pub", set = "pub")]
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

#[allow(non_snake_case)]
pub mod Body {
    use crate::types::{CorrectionClass, CreditType, OrderClass, Price, Quantity, TargetExchange};
    use getset::{Getters, Setters};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Getters, Setters, Serialize, Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub struct Order {
        #[getset(get = "pub", set = "pub")]
        /// 종합계좌번호(계좌번호 체계(8-2)의 앞 8자리)
        cano: String,
        #[getset(get = "pub", set = "pub")]
        /// 계좌상품코드(계좌번호 체계(8-2)의 뒤 2자리)
        acnt_prdt_cd: String,
        #[getset(get = "pub", set = "pub")]
        /// 종목코드(6자리)
        pdno: String,
        #[getset(get = "pub", set = "pub")]
        /// 주문구분
        ord_dvsn: OrderClass,
        #[getset(get = "pub", set = "pub")]
        /// 주문수량(주문주식수)
        ord_qty: Quantity,
        #[getset(get = "pub", set = "pub")]
        /// 주문단가(1주당 가격; 시장가는 0으로)
        ord_unpr: Price,
        /// 거래소ID구분코드(KRX/NXT/SOR)
        #[getset(get = "pub", set = "pub")]
        excg_id_dvsn_cd: TargetExchange,
    }

    impl Order {
        pub fn new(
            cano: String,
            acnt_prdt_cd: String,
            pdno: String,
            ord_dvsn: OrderClass,
            ord_qty: Quantity,
            ord_unpr: Price,
            excg_id_dvsn_cd: Option<TargetExchange>,
        ) -> Self {
            Self {
                cano,
                acnt_prdt_cd,
                pdno,
                ord_dvsn,
                ord_qty,
                ord_unpr,
                excg_id_dvsn_cd: excg_id_dvsn_cd.unwrap_or_default(),
            }
        }
        pub fn get_json_string(self) -> String {
            serde_json::json!(self).to_string()
        }
    }

    #[derive(Debug, Clone, PartialEq, Getters, Setters, Serialize, Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub struct Correction {
        /// 종합계좌번호(계좌번호 체계(8-2)의 앞 8자리)
        #[getset(get = "pub", set = "pub")]
        cano: String,
        /// 계좌상품코드(계좌번호 체계(8-2)의 뒤 2자리)
        #[getset(get = "pub", set = "pub")]
        acnt_prdt_cd: String,
        /// 한국거래소전송주문조직번호(주문시 한국투자증권 시스템에서
        /// 지정된 영업점코드)
        #[getset(get = "pub", set = "pub")]
        krx_fwdg_ord_orgno: String,
        /// 원주문번호(주식일별주문체결조회 API output1의 odno(주문번호) 값 입력.
        /// 주문시 한국투자증권 시스템에서 채번된 주문번호)
        #[getset(get = "pub", set = "pub")]
        orgn_odno: String,
        /// 주문구분
        #[getset(get = "pub", set = "pub")]
        ord_dvsn: OrderClass,
        /// 정정취소구분코드
        #[getset(get = "pub", set = "pub")]
        rvse_cncl_dvsn_cd: CorrectionClass,
        /// 주문수량(주문주식수)
        #[getset(get = "pub", set = "pub")]
        ord_qty: Quantity,
        /// 주문단가([정정] 정정주문 1주당 가격, [취소] "0")
        #[getset(get = "pub", set = "pub")]
        ord_unpr: Price,
        /// 잔량전부주문여부([정정/취소] Y: 잔량전부, N: 잔량일부)
        #[getset(get = "pub", set = "pub")]
        qty_all_ord_yn: char,
        /// 거래소ID구분코드(KRX/NXT/SOR)
        #[getset(get = "pub", set = "pub")]
        excg_id_dvsn_cd: TargetExchange,
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
            excg_id_dvsn_cd: Option<TargetExchange>,
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
                qty_all_ord_yn: if qty_all_ord_yn { 'Y' } else { 'N' },
                excg_id_dvsn_cd: excg_id_dvsn_cd.unwrap_or_default(),
            }
        }
        pub fn get_json_string(self) -> String {
            serde_json::json!(self).to_string()
        }
    }

    #[derive(Debug, Clone, PartialEq, Getters, Setters, Serialize, Deserialize)]
    pub struct CreditOrder {
        /// 종합계좌번호
        #[getset(get = "pub", set = "pub")]
        cano: String,
        /// 계좌상품코드
        #[getset(get = "pub", set = "pub")]
        acnt_prdt_cd: String,
        /// 상품번호
        #[getset(get = "pub", set = "pub")]
        pdno: String,
        /// 신용유형
        #[getset(get = "pub", set = "pub")]
        crdt_type: CreditType,
        /// 대출일자
        #[getset(get = "pub", set = "pub")]
        loan_at: String,
        /// 주문구분
        #[getset(get = "pub", set = "pub")]
        ord_dvsn: OrderClass,
        /// 주문수량
        #[getset(get = "pub", set = "pub")]
        ord_qty: Quantity,
        /// 주문단가
        #[getset(get = "pub", set = "pub")]
        ord_unpr: Price,
        /// 예약주문여부
        #[getset(get = "pub", set = "pub")]
        rsvn_ord_yn: Option<char>,
        /// 비상주문여부
        #[getset(get = "pub", set = "pub")]
        emgc_ord_yn: Option<char>,
        /// 프로그램매매구분
        #[getset(get = "pub", set = "pub")]
        pgtr_dvsn: Option<String>,
        ///운용사지정주문번호
        #[getset(get = "pub", set = "pub")]
        mgco_aptm_odno: Option<String>,
        /// 대량거래협상상세번호
        #[getset(get = "pub", set = "pub")]
        lqty_tr_ngtn_dtl_no: Option<char>,
        /// 대량거래협정번호
        #[getset(get = "pub", set = "pub")]
        lqty_tr_agmt_no: Option<String>,
        /// 대량거래협상자Id
        #[getset(get = "pub", set = "pub")]
        lqty_tr_ngtn_id: Option<String>,
        /// LP주문여부
        #[getset(get = "pub", set = "pub")]
        lp_ord_yn: Option<char>,
        /// 매체주문번호
        #[getset(get = "pub", set = "pub")]
        mdia_odno: Option<String>,
        /// 주문서버구분코드
        #[getset(get = "pub", set = "pub")]
        ord_svr_dvsn_cd: Option<String>,
        /// 프로그램호가신고구분코드
        #[getset(get = "pub", set = "pub")]
        pgm_nmpr_stmt_dvsn_cd: Option<String>,
        /// 반대매매선정사유코드
        #[getset(get = "pub", set = "pub")]
        cvrg_slct_rson_cd: Option<String>,
        /// 반대매매순번
        #[getset(get = "pub", set = "pub")]
        cvrg_seq: Option<String>,
        /// 거래소ID구분코드
        #[getset(get = "pub", set = "pub")]
        excg_id_dvsn_cd: TargetExchange,
        /// 조건가격
        #[getset(get = "pub", set = "pub")]
        cndt_pric: Option<Price>,
    }
    impl CreditOrder {
        pub fn new(
            cano: String,                          // 종합계좌번호
            acnt_prdt_cd: String,                  // 계좌상품코드
            pdno: String,                          // 상품번호
            crdt_type: CreditType,                 // 신용유형
            loan_at: String,                       // 대출일자
            ord_dvsn: OrderClass,                  // 주문구분
            ord_qty: Quantity,                     // 주문수량
            ord_unpr: Price,                       // 주문단가
            rsvn_ord_yn: Option<char>,             // 예약주문여부
            emgc_ord_yn: Option<char>,             // 비상주문여부
            pgtr_dvsn: Option<String>,             // 프로그램매매구분
            mgco_aptm_odno: Option<String>,        // 운용사지정주문번호
            lqty_tr_ngtn_dtl_no: Option<char>,     // 대량거래협상상세번호
            lqty_tr_agmt_no: Option<String>,       // 대량거래협정번호
            lqty_tr_ngtn_id: Option<String>,       // 대량거래협상자Id
            lp_ord_yn: Option<char>,               // LP주문여부
            mdia_odno: Option<String>,             // 매체주문번호
            ord_svr_dvsn_cd: Option<String>,       // 주문서버구분코드
            pgm_nmpr_stmt_dvsn_cd: Option<String>, // 프로그램호가신고구분코드
            cvrg_slct_rson_cd: Option<String>,     // 반대매매선정사유코드
            cvrg_seq: Option<String>,              // 반대매매순번
            excg_id_dvsn_cd: TargetExchange,       // 거래소ID구분코드
            cndt_pric: Option<Price>,              // 조건가격
        ) -> Self {
            Self {
                cano,
                acnt_prdt_cd,
                pdno,
                crdt_type,
                loan_at,
                ord_dvsn,
                ord_qty,
                ord_unpr,
                rsvn_ord_yn,
                emgc_ord_yn,
                pgtr_dvsn,
                mgco_aptm_odno,
                lqty_tr_ngtn_dtl_no,
                lqty_tr_agmt_no,
                lqty_tr_ngtn_id,
                lp_ord_yn,
                mdia_odno,
                ord_svr_dvsn_cd,
                pgm_nmpr_stmt_dvsn_cd,
                cvrg_slct_rson_cd,
                cvrg_seq,
                excg_id_dvsn_cd,
                cndt_pric,
            }
        }
        pub fn get_json_string(self) -> String {
            serde_json::json!(self).to_string()
        }
    }
}

#[allow(non_snake_case)]
pub mod Query {
    use getset::{Getters, Setters};
    use serde::{Deserialize, Serialize};

    /// 주식잔고조회 Query Parameter [v1_국내주식-006]
    #[derive(Debug, Clone, PartialEq, Getters, Setters, Serialize, Deserialize)]
    pub struct InquireBalance {
        /// 종합계좌번호 (계좌번호 체계(8-2)의 앞 8자리)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "CANO")]
        cano: String,
        /// 계좌상품코드 (계좌번호 체계(8-2)의 뒤 2자리)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "ACNT_PRDT_CD")]
        acnt_prdt_cd: String,
        /// 시간외단일가여부 (N: 기본값)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "AFHR_FLPR_YN")]
        afhr_flpr_yn: String,
        /// 오프라인여부 (공란: 기본값)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "OFL_YN")]
        ofl_yn: String,
        /// 조회구분 (01: 대출일별)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "INQR_DVSN")]
        inqr_dvsn: String,
        /// 단가구분 (01: 기본값)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "UNPR_DVSN")]
        unpr_dvsn: String,
        /// 펀드결제분포함여부 (N: 포함하지 않음)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "FUND_STTL_ICLD_YN")]
        fund_sttl_icld_yn: String,
        /// 융자금액자동상환여부 (N: 기본값)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "FNCG_AMT_AUTO_RDPT_YN")]
        fncg_amt_auto_rdpt_yn: String,
        /// 처리구분 (00: 전일매매포함, 01: 전일매매미포함)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "PRCS_DVSN")]
        prcs_dvsn: String,
        /// 연속조회검색조건100 (공란: 최초 조회)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "CTX_AREA_FK100")]
        ctx_area_fk100: String,
        /// 연속조회키100 (공란: 최초 조회)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "CTX_AREA_NK100")]
        ctx_area_nk100: String,
    }

    impl InquireBalance {
        pub fn new(
            cano: String,
            acnt_prdt_cd: String,
            afhr_flpr_yn: Option<String>,
            inqr_dvsn: Option<String>,
            prcs_dvsn: Option<String>,
            ctx_area_fk100: Option<String>,
            ctx_area_nk100: Option<String>,
        ) -> Self {
            Self {
                cano,
                acnt_prdt_cd,
                afhr_flpr_yn: afhr_flpr_yn.unwrap_or_else(|| "N".to_string()),
                ofl_yn: String::new(),
                inqr_dvsn: inqr_dvsn.unwrap_or_else(|| "01".to_string()),
                unpr_dvsn: "01".to_string(),
                fund_sttl_icld_yn: "N".to_string(),
                fncg_amt_auto_rdpt_yn: "N".to_string(),
                prcs_dvsn: prcs_dvsn.unwrap_or_else(|| "00".to_string()),
                ctx_area_fk100: ctx_area_fk100.unwrap_or_default(),
                ctx_area_nk100: ctx_area_nk100.unwrap_or_default(),
            }
        }

        pub fn into_iter(&self) -> [(&'static str, String); 11] {
            [
                ("CANO", self.cano.clone()),
                ("ACNT_PRDT_CD", self.acnt_prdt_cd.clone()),
                ("AFHR_FLPR_YN", self.afhr_flpr_yn.clone()),
                ("OFL_YN", self.ofl_yn.clone()),
                ("INQR_DVSN", self.inqr_dvsn.clone()),
                ("UNPR_DVSN", self.unpr_dvsn.clone()),
                ("FUND_STTL_ICLD_YN", self.fund_sttl_icld_yn.clone()),
                ("FNCG_AMT_AUTO_RDPT_YN", self.fncg_amt_auto_rdpt_yn.clone()),
                ("PRCS_DVSN", self.prcs_dvsn.clone()),
                ("CTX_AREA_FK100", self.ctx_area_fk100.clone()),
                ("CTX_AREA_NK100", self.ctx_area_nk100.clone()),
            ]
        }
    }

    /// 주식정정취소가능주문조회 Query Parameter [v1_국내주식-004]
    #[derive(Debug, Clone, PartialEq, Getters, Setters, Serialize, Deserialize)]
    pub struct InquirePsblRvsecncl {
        /// 종합계좌번호 (계좌번호 체계(8-2)의 앞 8자리)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "CANO")]
        cano: String,
        /// 계좌상품코드 (계좌번호 체계(8-2)의 뒤 2자리)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "ACNT_PRDT_CD")]
        acnt_prdt_cd: String,
        /// 연속조회검색조건100 (공란: 최초 조회)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "CTX_AREA_FK100")]
        ctx_area_fk100: String,
        /// 연속조회키100 (공란: 최초 조회)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "CTX_AREA_NK100")]
        ctx_area_nk100: String,
        /// 조회구분1 (0: 조회순서, 1: 주문순, 2: 종목순)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "INQR_DVSN_1")]
        inqr_dvsn_1: String,
        /// 조회구분2 (0: 전체, 1: 매도, 2: 매수)
        #[getset(get = "pub", set = "pub")]
        #[serde(rename = "INQR_DVSN_2")]
        inqr_dvsn_2: String,
    }

    impl InquirePsblRvsecncl {
        pub fn new(
            cano: String,
            acnt_prdt_cd: String,
            ctx_area_fk100: Option<String>,
            ctx_area_nk100: Option<String>,
            inqr_dvsn_1: Option<String>,
            inqr_dvsn_2: Option<String>,
        ) -> Self {
            Self {
                cano,
                acnt_prdt_cd,
                ctx_area_fk100: ctx_area_fk100.unwrap_or_default(),
                ctx_area_nk100: ctx_area_nk100.unwrap_or_default(),
                inqr_dvsn_1: inqr_dvsn_1.unwrap_or_else(|| "0".to_string()),
                inqr_dvsn_2: inqr_dvsn_2.unwrap_or_else(|| "0".to_string()),
            }
        }

        pub fn into_iter(&self) -> [(&'static str, String); 6] {
            [
                ("CANO", self.cano.clone()),
                ("ACNT_PRDT_CD", self.acnt_prdt_cd.clone()),
                ("CTX_AREA_FK100", self.ctx_area_fk100.clone()),
                ("CTX_AREA_NK100", self.ctx_area_nk100.clone()),
                ("INQR_DVSN_1", self.inqr_dvsn_1.clone()),
                ("INQR_DVSN_2", self.inqr_dvsn_2.clone()),
            ]
        }
    }
}

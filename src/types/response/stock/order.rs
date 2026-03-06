#![allow(non_snake_case)]
pub mod Header {
    use getset::Getters;
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct Stock {
        /// 거래ID(요청한 tr_id)
        #[getset(get = "pub")]
        tr_id: String,
        /// 연속 거래 여부(F or M: 다음 데이터 있음 / D or E: 마지막 데이터)
        #[getset(get = "pub")]
        tr_cont: String,
        /// Global UID(거래고유번호)
        #[getset(get = "pub")]
        gt_uid: String,
    }
}

pub mod Body {
    use super::Output;
    use getset::Getters;
    use serde::{Deserialize, Serialize};

    /// 주식주문(현금, 신용, 정정취소)
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct Order {
        /// 0: 성공, 0 이외의 값: 실패
        #[getset(get = "pub")]
        rt_cd: String,
        /// 응답코드
        #[getset(get = "pub")]
        msg_cd: String,
        /// 응답메시지
        #[getset(get = "pub")]
        msg1: String,
        /// 응답 상세
        #[getset(get = "pub")]
        output: Option<Output::Order>,
    }

    /// 주식정정취소가능주문조회
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct InquirePsblRvsecncl {
        /// 0: 성공, 0 이외의 값: 실패
        #[getset(get = "pub")]
        rt_cd: String,
        /// 응답코드
        #[getset(get = "pub")]
        msg_cd: String,
        /// 응답메시지
        #[getset(get = "pub")]
        msg1: String,
        /// 연속조회검색조건100
        #[getset(get = "pub")]
        ctx_area_fk100: Option<String>,
        /// 연속조회키100
        #[getset(get = "pub")]
        ctx_area_nk100: Option<String>,
        /// 응답 상세 목록
        #[getset(get = "pub")]
        output: Vec<Output::InquirePsblRvsecncl>,
    }
}

pub mod Output {
    use crate::types::OrderClass;
    use getset::Getters;
    use serde::{Deserialize, Serialize};

    /// 주식주문(현금, 신용, 정정취소)
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    #[serde(rename_all = "UPPERCASE")]
    pub struct Order {
        /// 주문시 한국투자증권 시스템에서 지정된 영업점코드
        #[getset(get = "pub")]
        krx_fwdg_ord_orgno: String,
        /// 주문번호(주문시 한국투자증권 시스템에서 채번된 주문번호)
        #[getset(get = "pub")]
        odno: String,
        /// 주문시각(시분초HHMMSS)
        #[getset(get = "pub")]
        ord_tmd: String,
    }

    /// 주식정정취소가능주문조회
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct InquirePsblRvsecncl {
        /// 주문채번지점번호(주문시 한국투자증권 시스템에서 지정된 영업점코드)
        #[getset(get = "pub")]
        ord_gno_brno: String,
        /// 주문번호(주문시 한국투자증권 시스템에서 채번된 주문번호)
        #[getset(get = "pub")]
        odno: String,
        /// 원주문번호(정정/취소주문인 경우 원주문번호)
        #[getset(get = "pub")]
        orgn_odno: String,
        /// 주문구분명
        #[getset(get = "pub")]
        ord_dvsn_name: String,
        /// 상품번호(종목번호(뒤 6자리만 해당))
        #[getset(get = "pub")]
        pdno: String,
        /// 상품명(종목명)
        #[getset(get = "pub")]
        prdt_name: String,
        /// 정정취소구분명(정정 또는 취소 여부 표시)
        #[getset(get = "pub")]
        rvse_cncl_dvsn_name: String,
        /// 주문수량
        #[getset(get = "pub")]
        ord_qty: String,
        /// 주문단가(1주당 주문가격)
        #[getset(get = "pub")]
        ord_unpr: String,
        /// 주문시각(시분초HHMMSS)
        #[getset(get = "pub")]
        ord_tmd: String,
        /// 총체결수량(주문 수량 중 체결된 수량)
        #[getset(get = "pub")]
        tot_ccld_qty: String,
        /// 총체결금액(주문금액 중 체결금액)
        #[getset(get = "pub")]
        tot_ccld_amt: String,
        /// 가능수량(정정/취소 주문 가능 수량)
        #[getset(get = "pub")]
        psbl_qty: String,
        /// 매도매수구분코드(01: 매도, 02: 매수)
        #[getset(get = "pub")]
        sll_buy_dvsn_cd: String,
        /// 주문구분코드
        #[getset(get = "pub")]
        ord_dvsn_cd: OrderClass,
        /// 운용사지정주문번호(주문번호(운용사 통한 주문))
        #[getset(get = "pub")]
        mgco_aptm_odno: String,
    }
}

/// 주식일별주문체결조회 response types [v1_국내주식-005]
pub mod daily_ccld {
    use getset::Getters;
    use serde::{Deserialize, Serialize};

    /// 주식일별주문체결조회 응답 Body
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct InquireDailyCcld {
        /// 0: 성공, 0 이외의 값: 실패
        #[getset(get = "pub")]
        rt_cd: String,
        /// 응답코드
        #[getset(get = "pub")]
        msg_cd: String,
        /// 응답메시지
        #[getset(get = "pub")]
        msg1: String,
        /// 연속조회검색조건100
        #[getset(get = "pub")]
        ctx_area_fk100: Option<String>,
        /// 연속조회키100
        #[getset(get = "pub")]
        ctx_area_nk100: Option<String>,
        /// 주문체결 목록
        #[getset(get = "pub")]
        output1: Vec<DailyCcldItem>,
        /// 합계 정보
        #[getset(get = "pub")]
        output2: Vec<DailyCcldSummary>,
    }

    /// output1 - 주문체결 상세
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct DailyCcldItem {
        /// 주문일자 (YYYYMMDD)
        #[getset(get = "pub")]
        ord_dt: String,
        /// 주문채번지점번호
        #[getset(get = "pub")]
        ord_gno_brno: String,
        /// 주문번호
        #[getset(get = "pub")]
        odno: String,
        /// 원주문번호
        #[getset(get = "pub")]
        orgn_odno: String,
        /// 주문구분명
        #[getset(get = "pub")]
        ord_dvsn_name: String,
        /// 매도매수구분코드 (01: 매도, 02: 매수)
        #[getset(get = "pub")]
        sll_buy_dvsn_cd: String,
        /// 매도매수구분코드명
        #[getset(get = "pub")]
        sll_buy_dvsn_cd_name: String,
        /// 상품번호 (종목코드)
        #[getset(get = "pub")]
        pdno: String,
        /// 상품명 (종목명)
        #[getset(get = "pub")]
        prdt_name: String,
        /// 주문수량
        #[getset(get = "pub")]
        ord_qty: String,
        /// 주문단가 (1주당 주문가격)
        #[getset(get = "pub")]
        ord_unpr: String,
        /// 주문시각 (HHMMSS)
        #[getset(get = "pub")]
        ord_tmd: String,
        /// 총체결수량
        #[getset(get = "pub")]
        tot_ccld_qty: String,
        /// 평균가
        #[getset(get = "pub")]
        avg_prvs: String,
        /// 취소여부 (Y: 취소, N: 정상)
        #[getset(get = "pub")]
        cncl_yn: String,
        /// 총체결금액
        #[getset(get = "pub")]
        tot_ccld_amt: String,
        /// 대출일자
        #[getset(get = "pub")]
        loan_dt: String,
        /// 주문구분코드
        #[getset(get = "pub")]
        ord_dvsn_cd: String,
        /// 취소확인수량
        #[getset(get = "pub")]
        cncl_cfrm_qty: String,
        /// 잔량수량
        #[getset(get = "pub")]
        rmn_qty: String,
        /// 거부수량
        #[getset(get = "pub")]
        rjct_qty: String,
        /// 체결조건명
        #[getset(get = "pub")]
        ccld_cndt_name: String,
        /// 조회IP주소
        #[getset(get = "pub")]
        inqr_ip_addr: String,
        /// 복합주문부모주문접수구분명
        #[getset(get = "pub")]
        cpbc_ordp_ord_rcit_dvsn_name: String,
        /// 복합주문부모알림방법명
        #[getset(get = "pub")]
        cpbc_ordp_infm_mthd_name: String,
        /// 알림시각
        #[getset(get = "pub")]
        infm_tmd: String,
        /// 연락전화번호
        #[getset(get = "pub")]
        ctac_tlno: String,
        /// 상품유형코드
        #[getset(get = "pub")]
        prdt_type_cd: String,
        /// 거래소구분코드
        #[getset(get = "pub")]
        excg_dvsn_cd: String,
    }

    /// output2 - 합계
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct DailyCcldSummary {
        /// 총주문수량
        #[getset(get = "pub")]
        tot_ord_qty: String,
        /// 총체결수량
        #[getset(get = "pub")]
        tot_ccld_qty: String,
        /// 총체결금액
        #[getset(get = "pub")]
        tot_ccld_amt: String,
        /// 추정제비용합계
        #[getset(get = "pub")]
        prsm_tlex_smtl: String,
        /// 매입평균가격
        #[getset(get = "pub")]
        pchs_avg_pric: String,
    }
}

/// 주식잔고조회 response types [v1_국내주식-006]
pub mod balance {
    use getset::Getters;
    use serde::{Deserialize, Serialize};

    /// 주식잔고조회 응답 Body
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct InquireBalance {
        /// 0: 성공, 0 이외의 값: 실패
        #[getset(get = "pub")]
        rt_cd: String,
        /// 응답코드
        #[getset(get = "pub")]
        msg_cd: String,
        /// 응답메시지
        #[getset(get = "pub")]
        msg1: String,
        /// 연속조회검색조건100
        #[getset(get = "pub")]
        ctx_area_fk100: Option<String>,
        /// 연속조회키100
        #[getset(get = "pub")]
        ctx_area_nk100: Option<String>,
        /// 보유종목 목록
        #[getset(get = "pub")]
        output1: Vec<BalanceItem>,
        /// 계좌 요약 정보
        #[getset(get = "pub")]
        output2: Vec<BalanceSummary>,
    }

    /// output1 - 보유종목 상세
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct BalanceItem {
        /// 상품번호 (종목번호 뒷 6자리)
        #[getset(get = "pub")]
        pdno: String,
        /// 상품명 (종목명)
        #[getset(get = "pub")]
        prdt_name: String,
        /// 매매구분명 (현금/신용 등)
        #[getset(get = "pub")]
        trad_dvsn_name: String,
        /// 전일매수수량
        #[getset(get = "pub")]
        bfdy_buy_qty: String,
        /// 전일매도수량
        #[getset(get = "pub")]
        bfdy_sll_qty: String,
        /// 금일매수수량
        #[getset(get = "pub")]
        thdt_buyqty: String,
        /// 금일매도수량
        #[getset(get = "pub")]
        thdt_sll_qty: String,
        /// 보유수량
        #[getset(get = "pub")]
        hldg_qty: String,
        /// 주문가능수량
        #[getset(get = "pub")]
        ord_psbl_qty: String,
        /// 매입평균가격
        #[getset(get = "pub")]
        pchs_avg_pric: String,
        /// 매입금액
        #[getset(get = "pub")]
        pchs_amt: String,
        /// 현재가
        #[getset(get = "pub")]
        prpr: String,
        /// 평가금액
        #[getset(get = "pub")]
        evlu_amt: String,
        /// 평가손익금액
        #[getset(get = "pub")]
        evlu_pfls_amt: String,
        /// 평가손익율
        #[getset(get = "pub")]
        evlu_pfls_rt: String,
        /// 평가수익율
        #[getset(get = "pub")]
        evlu_erng_rt: String,
        /// 대출일자
        #[getset(get = "pub")]
        loan_dt: String,
        /// 대출금액
        #[getset(get = "pub")]
        loan_amt: String,
        /// 대주매각대금
        #[getset(get = "pub")]
        stln_slng_chgs: String,
        /// 만기일자
        #[getset(get = "pub")]
        expd_dt: String,
        /// 등락율
        #[getset(get = "pub")]
        fltt_rt: String,
        /// 전일대비증감
        #[getset(get = "pub")]
        bfdy_cprs_icdc: String,
        /// 종목증거금율명
        #[getset(get = "pub")]
        item_mgna_rt_name: String,
        /// 보증금율명
        #[getset(get = "pub")]
        grta_rt_name: String,
        /// 대용가격
        #[getset(get = "pub")]
        sbst_pric: String,
        /// 주식대출단가
        #[getset(get = "pub")]
        stck_loan_unpr: String,
    }

    /// output2 - 계좌 요약
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct BalanceSummary {
        /// 예수금총금액
        #[getset(get = "pub")]
        dnca_tot_amt: String,
        /// 익일정산금액 (D+1 예수금)
        #[getset(get = "pub")]
        nxdy_excc_amt: String,
        /// 가수도정산금액 (D+2 예수금)
        #[getset(get = "pub")]
        prvs_rcdl_excc_amt: String,
        /// CMA평가금액
        #[getset(get = "pub")]
        cma_evlu_amt: String,
        /// 전일매수금액
        #[getset(get = "pub")]
        bfdy_buy_amt: String,
        /// 금일매수금액
        #[getset(get = "pub")]
        thdt_buy_amt: String,
        /// 익일자동상환금액
        #[getset(get = "pub")]
        nxdy_auto_rdpt_amt: String,
        /// 전일매도금액
        #[getset(get = "pub")]
        bfdy_sll_amt: String,
        /// 금일매도금액
        #[getset(get = "pub")]
        thdt_sll_amt: String,
        /// D+2자동상환금액
        #[getset(get = "pub")]
        d2_auto_rdpt_amt: String,
        /// 전일제비용금액
        #[getset(get = "pub")]
        bfdy_tlex_amt: String,
        /// 금일제비용금액
        #[getset(get = "pub")]
        thdt_tlex_amt: String,
        /// 총대출금액
        #[getset(get = "pub")]
        tot_loan_amt: String,
        /// 유가평가금액
        #[getset(get = "pub")]
        scts_evlu_amt: String,
        /// 총평가금액 (유가증권 평가금액 + D+2 예수금)
        #[getset(get = "pub")]
        tot_evlu_amt: String,
        /// 순자산금액
        #[getset(get = "pub")]
        nass_amt: String,
        /// 융자금자동상환여부
        #[getset(get = "pub")]
        fncg_gld_auto_rdpt_yn: String,
        /// 매입금액합계금액
        #[getset(get = "pub")]
        pchs_amt_smtl_amt: String,
        /// 평가금액합계금액
        #[getset(get = "pub")]
        evlu_amt_smtl_amt: String,
        /// 평가손익합계금액
        #[getset(get = "pub")]
        evlu_pfls_smtl_amt: String,
        /// 총대주매각대금
        #[getset(get = "pub")]
        tot_stln_slng_chgs: String,
        /// 전일총자산평가금액
        #[getset(get = "pub")]
        bfdy_tot_asst_evlu_amt: String,
        /// 자산증감액
        #[getset(get = "pub")]
        asst_icdc_amt: String,
        /// 자산증감수익율
        #[getset(get = "pub")]
        asst_icdc_erng_rt: String,
    }
}

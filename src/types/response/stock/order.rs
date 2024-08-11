#![allow(non_snake_case)]
pub mod Header {
    use getset::Getters;
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct Stock {
        #[getset(get = "pub")]
        /// 거래ID(요청한 tr_id)
        tr_id: String,
        #[getset(get = "pub")]
        /// 연속 거래 여부(F or M: 다음 데이터 있음 / D or E: 마지막 데이터)
        tr_cont: String,
        #[getset(get = "pub")]
        /// Global UID(거래고유번호)
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
        #[getset(get = "pub")]
        /// 0: 성공, 0 이외의 값: 실패
        rt_cd: String,
        #[getset(get = "pub")]
        /// 응답코드
        msg_cd: String,
        #[getset(get = "pub")]
        /// 응답메시지
        msg1: String,
        #[getset(get = "pub")]
        /// 응답 상세
        output: Option<Output::Order>,
    }

    /// 주식정정취소가능주문조회
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct InquirePsblRvsecncl {
        #[getset(get = "pub")]
        /// 0: 성공, 0 이외의 값: 실패
        tr_cd: String,
        #[getset(get = "pub")]
        /// 응답코드
        msg_cd: String,
        #[getset(get = "pub")]
        /// 응답메시지
        msg1: String,
        #[getset(get = "pub")]
        /// 연속조회검색조건100
        ctx_area_fk100: Option<String>,
        #[getset(get = "pub")]
        /// 연속조회키100
        ctx_area_nk100: Option<String>,
        #[getset(get = "pub")]
        /// 응답 상세
        output: Output::InquirePsblRvsecncl,
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
        #[getset(get = "pub")]
        /// 주문시 한국투자증권 시스템에서 지정된 영업점코드
        krx_fwdg_ord_orgno: String,
        #[getset(get = "pub")]
        /// 주문번호(주문시 한국투자증권 시스템에서 채번된 주문번호)
        odno: String,
        #[getset(get = "pub")]
        /// 주문시각(시분초HHMMSS)
        ord_tmd: String,
    }

    /// 주식정정취소가능주문조회
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct InquirePsblRvsecncl {
        #[getset(get = "pub")]
        /// 주문채번지점번호(주문시 한국투자증권 시스템에서 지정된 영업점코드)
        ord_gno_brno: String,
        #[getset(get = "pub")]
        /// 주문번호(주문시 한국투자증권 시스템에서 채번된 주문번호)
        odno: String,
        #[getset(get = "pub")]
        /// 원주문번호(정정/취소주문인 경우 원주문번호)
        orgn_odno: String,
        #[getset(get = "pub")]
        /// 주문구분명
        ord_dvsn_name: String,
        #[getset(get = "pub")]
        /// 상품번호(종목번호(뒤 6자리만 해당))
        pdno: String,
        #[getset(get = "pub")]
        /// 상품명(종목명)
        prdt_name: String,
        #[getset(get = "pub")]
        /// 정정취소구분명(정정 또는 취소 여부 표시)
        rvse_cncl_dvsn_name: String,
        #[getset(get = "pub")]
        /// 주문수량
        ord_qty: String,
        #[getset(get = "pub")]
        /// 주문단가(1주당 주문가격)
        ord_unpr: String,
        #[getset(get = "pub")]
        /// 주문시각(시분초HHMMSS)
        ord_tmd: String,
        #[getset(get = "pub")]
        /// 총체결수량(주문 수량 중 체결된 수량)
        tot_ccld_qty: String,
        #[getset(get = "pub")]
        /// 총체결금액(주문금액 중 체결금액)
        tot_ccld_amt: String,
        #[getset(get = "pub")]
        /// 가능수량(정정/취소 주문 가능 수량)
        psbl_qty: String,
        #[getset(get = "pub")]
        /// 매도매수구분코드(01: 매도, 02: 매수)
        sll_buy_dvsn_cd: String,
        #[getset(get = "pub")]
        /// 주문구분코드
        ord_dvsn_cd: OrderClass,
        #[getset(get = "pub")]
        /// 운용사지정주문번호(주문번호(운용사 통한 주문))
        mgco_aptm_odno: String,
    }
}

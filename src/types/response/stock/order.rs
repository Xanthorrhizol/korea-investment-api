#![allow(non_snake_case)]
pub mod Header {
    use getset::Getters;
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct Stock {
        #[getset(get = "pub")]
        tr_id: String, // 거래ID(요청한 tr_id)
        #[getset(get = "pub")]
        tr_cont: String, // 연속 거래 여부(F or M: 다음 데이터 있음 / D or E: 마지막 데이터)
        #[getset(get = "pub")]
        gt_uid: String, // Global UID(거래고유번호)
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
        rt_cd: String, // 0: 성공, 0 이외의 값: 실패
        #[getset(get = "pub")]
        msg_cd: String, // 응답코드
        #[getset(get = "pub")]
        msg1: String, // 응답메시지
        #[getset(get = "pub")]
        output: Option<Output::Order>, // 응답 상세
    }

    /// 주식정정취소가능주문조회
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct InquirePsblRvsecncl {
        #[getset(get = "pub")]
        tr_cd: String, // 0: 성공, 0 이외의 값: 실패
        #[getset(get = "pub")]
        msg_cd: String, // 응답코드
        #[getset(get = "pub")]
        msg1: String, // 응답메시지
        #[getset(get = "pub")]
        ctx_area_fk100: Option<String>, // 연속조회검색조건100
        #[getset(get = "pub")]
        ctx_area_nk100: Option<String>, // 연속조회키100
        #[getset(get = "pub")]
        output: Output::InquirePsblRvsecncl, // 응답 상세
    }
}

pub mod Output {
    use getset::Getters;
    use serde::{Deserialize, Serialize};

    /// 주식주문(현금, 신용, 정정취소)
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    #[serde(rename_all = "UPPERCASE")]
    pub struct Order {
        #[getset(get = "pub")]
        krx_fwdg_ord_orgno: String, // 주문시 한국투자증권 시스템에서 지정된 영업점코드
        #[getset(get = "pub")]
        odno: String, // 주문번호(주문시 한국투자증권 시스템에서 채번된 주문번호)
        #[getset(get = "pub")]
        ord_tmd: String, // 주문시각(시분초HHMMSS)
    }

    /// 주식정정취소가능주문조회
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
    pub struct InquirePsblRvsecncl {
        #[getset(get = "pub")]
        ord_gno_brno: String, // 주문채번지점번호(주문시 한국투자증권 시스템에서 지정된 영업점코드)
        #[getset(get = "pub")]
        odno: String, // 주문번호(주문시 한국투자증권 시스템에서 채번된 주문번호)
        #[getset(get = "pub")]
        orgn_odno: String, // 원주문번호(정정/취소주문인 경우 원주문번호)
        #[getset(get = "pub")]
        ord_dvsn_name: String, // 주문구분명
        #[getset(get = "pub")]
        pdno: String, // 상품번호(종목번호(뒤 6자리만 해당))
        #[getset(get = "pub")]
        prdt_name: String, // 상품명(종목명)
        #[getset(get = "pub")]
        rvse_cncl_dvsn_name: String, // 정정취소구분명(정정 또는 취소 여부 표시)
        #[getset(get = "pub")]
        ord_qty: String, // 주문수량
        #[getset(get = "pub")]
        ord_unpr: String, // 주문단가(1주당 주문가격)
        #[getset(get = "pub")]
        ord_tmd: String, // 주문시각(시분초HHMMSS)
        #[getset(get = "pub")]
        tot_ccld_qty: String, // 총체결수량(주문 수량 중 체결된 수량)
        #[getset(get = "pub")]
        tot_ccld_amt: String, // 총체결금액(주문금액 중 체결금액)
        #[getset(get = "pub")]
        psbl_qty: String, // 가능수량(정정/취소 주문 가능 수량)
        #[getset(get = "pub")]
        sll_buy_dvsn_cd: String, // 매도매수구분코드(01: 매도, 02: 매수)
        #[getset(get = "pub")]
        ord_dvsn_cd: String, // 주문구분코드(
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
        #[getset(get = "pub")]
        mgco_aptm_odno: String, // 운용사지정주문번호(주문번호(운용사 통한 주문))
    }
}

use getset::Getters;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Getters)]
pub struct DailyPriceResponse {
    #[getset(get = "pub")]
    rt_cd: String, // 0: 성공, 0 이외의 값: 실패
    #[getset(get = "pub")]
    msg_cd: String, // 응답코드
    #[getset(get = "pub")]
    msg1: String, // 응답메시지
    #[getset(get = "pub")]
    output: Option<Vec<output::DailyPrice>>, // 응답 상세
}

pub mod output {
    use crate::types::{ExCode, VsPriceSign};
    use getset::Getters;
    use serde::Deserialize;

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct DailyPrice {
        #[getset(get = "pub")]
        stck_bsop_date: String, // 주식 영업 일자
        #[getset(get = "pub")]
        stck_oprc: String, // 주식 시가
        #[getset(get = "pub")]
        stck_hgpr: String, // 주식 최고가
        #[getset(get = "pub")]
        stck_lwpr: String, // 주식 최저가
        #[getset(get = "pub")]
        stck_clpr: String, // 주식 종가
        #[getset(get = "pub")]
        acml_vol: String, // 누적 거래량
        #[getset(get = "pub")]
        prdy_vrss_vol_rate: String, // 전일 대비 거래량 비율
        #[getset(get = "pub")]
        prdy_vrss: String, // 전일 대비
        #[getset(get = "pub")]
        prdy_vrss_sign: VsPriceSign, // 전일 대비 부호
        #[getset(get = "pub")]
        prdy_ctrt: String, // 전일 대비율
        #[getset(get = "pub")]
        hts_frgn_ehrt: String, // HTS 외국인 소진율
        #[getset(get = "pub")]
        frgn_ntby_qty: String, // 외국인 순매수 수량
        #[getset(get = "pub")]
        flng_cls_code: ExCode, // 락 구분 코드
        #[getset(get = "pub")]
        acml_prtt_rate: String, // 누적 분할 비율
    }
}

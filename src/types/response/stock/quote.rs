use getset::Getters;
use serde::Deserialize;

pub type DailyPriceResponse = QuoteResponse<output::DailyPrice, output::DailyPrice>;
pub type VolumeRankResponse = QuoteResponse<output::VolumeRank, output::VolumeRank>;
pub type GroupListResponse = QuoteResponse<output::GroupList, output::GroupList>;
pub type GroupItemResponse = QuoteResponse<output::GroupInfo, output::GroupItem>;

#[derive(Clone, Debug, Deserialize, Getters)]
pub struct QuoteResponse<T, R> {
    #[getset(get = "pub")]
    rt_cd: String, // 0: 성공, 0 이외의 값: 실패
    #[getset(get = "pub")]
    msg_cd: String, // 응답코드
    #[getset(get = "pub")]
    msg1: String, // 응답메시지
    #[getset(get = "pub")]
    output: Option<Vec<T>>, // 응답 상세
    #[getset(get = "pub")]
    output1: Option<T>, // 응답 상세1
    #[getset(get = "pub")]
    output2: Option<Vec<R>>, // 응답 상세2
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

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct VolumeRank {
        #[getset(get = "pub")]
        hts_kor_isnm: String, // HTS 한글 종목명
        #[getset(get = "pub")]
        mksc_shrn_iscd: String, // 유가증권 단축 종목코드
        #[getset(get = "pub")]
        data_rank: String, // 데이터 순위
        #[getset(get = "pub")]
        stck_prpr: String, // 주식 현재가
        #[getset(get = "pub")]
        prdy_vrss_sign: VsPriceSign, // 전일 대비 부호
        #[getset(get = "pub")]
        prdy_vrss: String, // 전일 대비
        #[getset(get = "pub")]
        prdy_ctrt: String, // 전일 대비율
        #[getset(get = "pub")]
        acml_vol: String, // 누적 거래량
        #[getset(get = "pub")]
        prdy_vol: String, // 전일 거래량
        #[getset(get = "pub")]
        lstn_stcn: String, // 상장 주수
        #[getset(get = "pub")]
        avrg_vol: String, // 평균 거래량
        #[getset(get = "pub")]
        n_befr_clpr_vrss_prpr_rate: String, // N일전종가대비현재가대비율
        #[getset(get = "pub")]
        vol_inrt: String, // 거래량 증가율
        #[getset(get = "pub")]
        vol_tnrt: String, // 거래량 회전율
        #[getset(get = "pub")]
        nday_vol_tnrt: String, // N일 거래량 회전율
        #[getset(get = "pub")]
        avrg_tr_pbmn: String, // 평균 거래 대금
        #[getset(get = "pub")]
        tr_pbmn_tnrt: String, // 거래대금회전율
        #[getset(get = "pub")]
        nday_tr_pbmn_tnrt: String, // N일 거래대금 회전율
        #[getset(get = "pub")]
        acml_tr_pbmn: String, // 누적 거래 대금
    }

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct GroupList {
        /// 일자
        #[getset(get = "pub")]
        date: String,
        /// 전송 시간
        #[getset(get = "pub")]
        tmm_hour: Option<String>,
        /// 데이터 순위
        #[getset(get = "pub")]
        data_rank: String,
        /// 관심 그룹 코드
        #[getset(get = "pub")]
        inter_grp_code: String,
        /// 관심 그룹 명
        #[getset(get = "pub")]
        inter_grp_name: String,
        /// 요청 개수
        #[getset(get = "pub")]
        ask_cnt: String,
    }

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct GroupInfo {
        /// 데이터 순위
        #[getset(get = "pub")]
        data_rank: String,
        /// 관심 그룹 코드
        #[getset(get = "pub")]
        inter_grp_code: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct GroupItem {
        /// FID 시장 구분 코드
        #[getset(get = "pub")]
        fid_mrkt_cls_code: String,
        /// 데이터 순위
        #[getset(get = "pub")]
        data_rank: String,
        /// 거래소 코드
        #[getset(get = "pub")]
        exch_code: String,
        /// 종목 코드
        #[getset(get = "pub")]
        jong_code: String,
        /// 색상 코드
        #[getset(get = "pub")]
        color_code: String,
        /// 메모
        #[getset(get = "pub")]
        memo: String,
        /// HTS 한글 종목명
        #[getset(get = "pub")]
        hts_kor_isnm: String,
        /// 기준일 순매수 수량
        #[getset(get = "pub")]
        fxdt_ntby_qty: String,
        /// 체결단가
        #[getset(get = "pub")]
        cntg_unpr: String,
        /// 체결 구분 코드
        #[getset(get = "pub")]
        cntg_cls_code: String,
    }
}

use getset::Getters;
use serde::Deserialize;

pub type DailyPriceResponse = QuoteResponse<Vec<output::DailyPrice>, (), ()>;
pub type VolumeRankResponse = QuoteResponse<Vec<output::VolumeRank>, (), ()>;
pub type GroupListResponse = QuoteResponse<Vec<output::GroupList>, (), Vec<output::GroupList>>;
pub type GroupItemResponse = QuoteResponse<(), output::GroupInfo, Vec<output::GroupItem>>;

#[derive(Clone, Debug, Deserialize, Getters)]
pub struct QuoteResponse<A, B, C> {
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
    output: Option<A>,
    #[getset(get = "pub")]
    /// 응답 상세1
    output1: Option<B>,
    #[getset(get = "pub")]
    /// 응답 상세2
    output2: Option<C>,
}

pub mod output {
    use crate::types::{ExCode, VsPriceSign};
    use getset::Getters;
    use serde::Deserialize;

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct DailyPrice {
        #[getset(get = "pub")]
        /// 주식 영업 일자
        stck_bsop_date: String,
        #[getset(get = "pub")]
        /// 주식 시가
        stck_oprc: String,
        #[getset(get = "pub")]
        /// 주식 최고가
        stck_hgpr: String,
        #[getset(get = "pub")]
        /// 주식 최저가
        stck_lwpr: String,
        #[getset(get = "pub")]
        /// 주식 종가
        stck_clpr: String,
        #[getset(get = "pub")]
        /// 누적 거래량
        acml_vol: String,
        #[getset(get = "pub")]
        /// 전일 대비 거래량 비율
        prdy_vrss_vol_rate: String,
        #[getset(get = "pub")]
        /// 전일 대비
        prdy_vrss: String,
        #[getset(get = "pub")]
        /// 전일 대비 부호
        prdy_vrss_sign: VsPriceSign,
        #[getset(get = "pub")]
        /// 전일 대비율
        prdy_ctrt: String,
        #[getset(get = "pub")]
        /// HTS 외국인 소진율
        hts_frgn_ehrt: String,
        #[getset(get = "pub")]
        /// 외국인 순매수 수량
        frgn_ntby_qty: String,
        #[getset(get = "pub")]
        /// 락 구분 코드
        flng_cls_code: ExCode,
        #[getset(get = "pub")]
        /// 누적 분할 비율
        acml_prtt_rate: String,
    }

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct VolumeRank {
        #[getset(get = "pub")]
        /// HTS 한글 종목명
        hts_kor_isnm: String,
        #[getset(get = "pub")]
        /// 유가증권 단축 종목코드
        mksc_shrn_iscd: String,
        #[getset(get = "pub")]
        /// 데이터 순위
        data_rank: String,
        #[getset(get = "pub")]
        /// 주식 현재가
        stck_prpr: String,
        #[getset(get = "pub")]
        /// 전일 대비 부호
        prdy_vrss_sign: VsPriceSign,
        #[getset(get = "pub")]
        /// 전일 대비
        prdy_vrss: String,
        #[getset(get = "pub")]
        /// 전일 대비율
        prdy_ctrt: String,
        #[getset(get = "pub")]
        /// 누적 거래량
        acml_vol: String,
        #[getset(get = "pub")]
        /// 전일 거래량
        prdy_vol: String,
        #[getset(get = "pub")]
        /// 상장 주수
        lstn_stcn: String,
        #[getset(get = "pub")]
        /// 평균 거래량
        avrg_vol: String,
        #[getset(get = "pub")]
        /// N일전종가대비현재가대비율
        n_befr_clpr_vrss_prpr_rate: String,
        #[getset(get = "pub")]
        /// 거래량 증가율
        vol_inrt: String,
        #[getset(get = "pub")]
        /// 거래량 회전율
        vol_tnrt: String,
        #[getset(get = "pub")]
        /// N일 거래량 회전율
        nday_vol_tnrt: String,
        #[getset(get = "pub")]
        /// 평균 거래 대금
        avrg_tr_pbmn: String,
        #[getset(get = "pub")]
        /// 거래대금회전율
        tr_pbmn_tnrt: String,
        #[getset(get = "pub")]
        /// N일 거래대금 회전율
        nday_tr_pbmn_tnrt: String,
        #[getset(get = "pub")]
        /// 누적 거래 대금
        acml_tr_pbmn: String,
    }

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct GroupList {
        #[getset(get = "pub")]
        /// 일자
        date: String,
        #[getset(get = "pub")]
        /// 전송 시간
        tmm_hour: Option<String>,
        #[getset(get = "pub")]
        /// 데이터 순위
        data_rank: String,
        #[getset(get = "pub")]
        /// 관심 그룹 코드
        inter_grp_code: String,
        #[getset(get = "pub")]
        /// 관심 그룹 명
        inter_grp_name: String,
        #[getset(get = "pub")]
        /// 요청 개수
        ask_cnt: String,
    }

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct GroupInfo {
        #[getset(get = "pub")]
        /// 데이터 순위
        data_rank: String,
        #[getset(get = "pub")]
        /// 관심 그룹 코드
        inter_grp_code: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct GroupItem {
        #[getset(get = "pub")]
        /// FID 시장 구분 코드
        fid_mrkt_cls_code: String,
        #[getset(get = "pub")]
        /// 데이터 순위
        data_rank: String,
        #[getset(get = "pub")]
        /// 거래소 코드
        exch_code: String,
        #[getset(get = "pub")]
        /// 종목 코드
        jong_code: String,
        #[getset(get = "pub")]
        /// 색상 코드
        color_code: String,
        #[getset(get = "pub")]
        /// 메모
        memo: String,
        #[getset(get = "pub")]
        /// HTS 한글 종목명
        hts_kor_isnm: String,
        #[getset(get = "pub")]
        /// 기준일 순매수 수량
        fxdt_ntby_qty: String,
        #[getset(get = "pub")]
        /// 체결단가
        cntg_unpr: String,
        #[getset(get = "pub")]
        /// 체결 구분 코드
        cntg_cls_code: String,
    }
}

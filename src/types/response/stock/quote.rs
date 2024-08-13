use getset::Getters;
use serde::Deserialize;

pub type DailyPriceResponse = QuoteResponse<Vec<output::DailyPrice>, (), ()>;
pub type VolumeRankResponse = QuoteResponse<Vec<output::VolumeRank>, (), ()>;
pub type GroupListResponse = QuoteResponse<Vec<output::GroupList>, (), Vec<output::GroupList>>;
pub type GroupItemResponse = QuoteResponse<(), output::GroupInfo, Vec<output::GroupItem>>;
pub type BasicStockInfoResponse = QuoteResponse<output::BasicStockInfo, (), ()>;

#[derive(Clone, Debug, Deserialize, Getters)]
pub struct QuoteResponse<A, B, C> {
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
    output: Option<A>,
    /// 응답 상세1
    #[getset(get = "pub")]
    output1: Option<B>,
    /// 응답 상세2
    #[getset(get = "pub")]
    output2: Option<C>,
}

pub mod output {
    use crate::types::{
        ExCode, ExchangeCode, MarketId, SecurityGroupId, StockKindCode, VsPriceSign,
    };
    use getset::Getters;
    use serde::Deserialize;

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct DailyPrice {
        /// 주식 영업 일자
        #[getset(get = "pub")]
        stck_bsop_date: String,
        /// 주식 시가
        #[getset(get = "pub")]
        stck_oprc: String,
        /// 주식 최고가
        #[getset(get = "pub")]
        stck_hgpr: String,
        /// 주식 최저가
        #[getset(get = "pub")]
        stck_lwpr: String,
        /// 주식 종가
        #[getset(get = "pub")]
        stck_clpr: String,
        /// 누적 거래량
        #[getset(get = "pub")]
        acml_vol: String,
        /// 전일 대비 거래량 비율
        #[getset(get = "pub")]
        prdy_vrss_vol_rate: String,
        /// 전일 대비
        #[getset(get = "pub")]
        prdy_vrss: String,
        /// 전일 대비 부호
        #[getset(get = "pub")]
        prdy_vrss_sign: VsPriceSign,
        /// 전일 대비율
        #[getset(get = "pub")]
        prdy_ctrt: String,
        /// HTS 외국인 소진율
        #[getset(get = "pub")]
        hts_frgn_ehrt: String,
        /// 외국인 순매수 수량
        #[getset(get = "pub")]
        frgn_ntby_qty: String,
        /// 락 구분 코드
        #[getset(get = "pub")]
        flng_cls_code: ExCode,
        /// 누적 분할 비율
        #[getset(get = "pub")]
        acml_prtt_rate: String,
    }

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct VolumeRank {
        /// HTS 한글 종목명
        #[getset(get = "pub")]
        hts_kor_isnm: String,
        /// 유가증권 단축 종목코드
        #[getset(get = "pub")]
        mksc_shrn_iscd: String,
        /// 데이터 순위
        #[getset(get = "pub")]
        data_rank: String,
        /// 주식 현재가
        #[getset(get = "pub")]
        stck_prpr: String,
        /// 전일 대비 부호
        #[getset(get = "pub")]
        prdy_vrss_sign: VsPriceSign,
        /// 전일 대비
        #[getset(get = "pub")]
        prdy_vrss: String,
        /// 전일 대비율
        #[getset(get = "pub")]
        prdy_ctrt: String,
        /// 누적 거래량
        #[getset(get = "pub")]
        acml_vol: String,
        /// 전일 거래량
        #[getset(get = "pub")]
        prdy_vol: String,
        /// 상장 주수
        #[getset(get = "pub")]
        lstn_stcn: String,
        /// 평균 거래량
        #[getset(get = "pub")]
        avrg_vol: String,
        /// N일전종가대비현재가대비율
        #[getset(get = "pub")]
        n_befr_clpr_vrss_prpr_rate: String,
        /// 거래량 증가율
        #[getset(get = "pub")]
        vol_inrt: String,
        /// 거래량 회전율
        #[getset(get = "pub")]
        vol_tnrt: String,
        /// N일 거래량 회전율
        #[getset(get = "pub")]
        nday_vol_tnrt: String,
        /// 평균 거래 대금
        #[getset(get = "pub")]
        avrg_tr_pbmn: String,
        /// 거래대금회전율
        #[getset(get = "pub")]
        tr_pbmn_tnrt: String,
        /// N일 거래대금 회전율
        #[getset(get = "pub")]
        nday_tr_pbmn_tnrt: String,
        /// 누적 거래 대금
        #[getset(get = "pub")]
        acml_tr_pbmn: String,
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

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct BasicStockInfo {
        ///상품번호
        #[getset(get = "pub")]
        pdno: String,
        /// 상품유형코드
        #[getset(get = "pub")]
        prdt_type_cd: String,
        /// 시장ID코드
        #[getset(get = "pub")]
        mket_id_cd: MarketId,
        /// 증권그룹ID코드
        #[getset(get = "pub")]
        scty_grp_id_cd: SecurityGroupId,
        /// 거래소구분코드
        #[getset(get = "pub")]
        excg_dvsn_cd: ExchangeCode,
        /// 결산월일
        #[getset(get = "pub")]
        setl_mmdd: String,
        /// 상장주수
        #[getset(get = "pub")]
        lstg_stqty: Option<String>,
        /// 상장자본금액
        #[getset(get = "pub")]
        lstg_cptl_amt: String,
        /// 자본금
        #[getset(get = "pub")]
        cpta: String,
        /// 액면가
        #[getset(get = "pub")]
        papr: String,
        /// 발행가격
        #[getset(get = "pub")]
        issu_pric: String,
        /// 코스피200종목여부
        #[getset(get = "pub")]
        kospi200_item_yn: String,
        /// 유가증권시장상장일자(YYYYmmdd)
        #[getset(get = "pub")]
        scts_mket_lstg_dt: String,
        /// 유가증권시장상장폐지일자(YYYYmmdd)
        #[getset(get = "pub")]
        scts_mket_lstg_abol_dt: String,
        /// 코스닥시장상장일자(YYYYmmdd)
        #[getset(get = "pub")]
        kosdaq_mket_lstg_dt: String,
        /// 코스닥시장상장폐지일자(YYYYmmdd)
        #[getset(get = "pub")]
        kosdaq_mket_lstg_abol_dt: String,
        /// 프리보드시장상장일자(YYYYmmdd)
        #[getset(get = "pub")]
        frbd_mket_lstg_dt: String,
        /// 프리보드시장상장폐지일자(YYYYmmdd)
        #[getset(get = "pub")]
        frbd_mket_lstg_abol_dt: String,
        /// 리츠종류코드
        #[getset(get = "pub")]
        reits_kind_cd: String,
        /// ETF구분코드
        #[getset(get = "pub")]
        etf_dvsn_cd: String,
        /// 유전펀드여부
        #[getset(get = "pub")]
        oilf_fund_yn: String,
        /// 지수업종대분류코드
        #[getset(get = "pub")]
        idx_bztp_lcls_cd: String,
        /// 지수업종중분류코드
        #[getset(get = "pub")]
        idx_bztp_mcls_cd: String,
        /// 지수업종소분류코드
        #[getset(get = "pub")]
        idx_bztp_scls_cd: String,
        /// 주식종류코드
        #[getset(get = "pub")]
        stck_kind_cd: StockKindCode,
        /// 뮤추얼펀드개시일자(YYYYmmdd)
        #[getset(get = "pub")]
        mfnd_opng_dt: String,
        /// 뮤추얼펀드종료일자(YYYYmmdd)
        #[getset(get = "pub")]
        mfnd_end_dt: String,
        /// 예탁등록취소일자(YYYYmmdd)
        #[getset(get = "pub")]
        dpsi_erlm_cncl_dt: String,
        /// ETFCU수량
        #[getset(get = "pub")]
        etf_cu_qty: String,
        /// 상품명
        #[getset(get = "pub")]
        prdt_name: String,
        /// 상품명120
        #[getset(get = "pub")]
        prdt_name120: String,
        /// 상품약어명
        #[getset(get = "pub")]
        prdt_abrv_name: String,
        /// 표준상품번호(ISIN)
        #[getset(get = "pub")]
        std_pdno: String,
        /// 상품영문명
        #[getset(get = "pub")]
        prdt_eng_name: String,
        /// 상품영문명120
        #[getset(get = "pub")]
        prdt_eng_name120: String,
        /// 상품영문약어명
        #[getset(get = "pub")]
        prdt_eng_abrv_name: String,
        /// 예탁지정등록여부
        #[getset(get = "pub")]
        dpsi_aptm_erlm_yn: String,
        /// ETF과세유형코드
        #[getset(get = "pub")]
        etf_txtn_type_cd: String,
        /// ETF유형코드
        #[getset(get = "pub")]
        etf_type_cd: String,
        /// 상장폐지일자(YYYYmmdd)
        #[getset(get = "pub")]
        lstg_abol_dt: String,
        /// 신주구주구분코드
        #[getset(get = "pub")]
        nwst_odst_dvsn_cd: String,
        /// 대용가격
        #[getset(get = "pub")]
        sbst_pric: String,
        /// 당사대용가격
        #[getset(get = "pub")]
        thco_sbst_pric: String,
        /// 당사대용가격변경일자(YYYYmmdd)
        #[getset(get = "pub")]
        thco_sbst_pric_chng_dt: String,
        /// 거래정지여부
        #[getset(get = "pub")]
        tr_stop_yn: String,
        /// 관리종목여부
        #[getset(get = "pub")]
        admn_item_yn: String,
        /// 당일종가
        #[getset(get = "pub")]
        thdt_clpr: String,
        /// 전일종가
        #[getset(get = "pub")]
        bfdy_clpr: String,
        /// 종가변경일자(YYYYmmdd)
        #[getset(get = "pub")]
        clpr_chng_dt: String,
        /// 표준산업분류코드
        #[getset(get = "pub")]
        std_idst_clsf_cd: String,
        /// 표준산업분류코드명
        #[getset(get = "pub")]
        std_idst_clsf_cd_name: String,
        /// 지수업종대분류코드명
        #[getset(get = "pub")]
        idx_bztp_lcls_cd_name: String,
        /// 지수업종중분류코드명
        #[getset(get = "pub")]
        idx_bztp_mcls_cd_name: String,
        /// 지수업종소분류코드명
        #[getset(get = "pub")]
        idx_bztp_scls_cd_name: String,
        /// OCR번호
        #[getset(get = "pub")]
        ocr_no: String,
        /// 크라우드펀딩종목여부
        #[getset(get = "pub")]
        crfd_item_yn: String,
        /// 전자증권여부
        #[getset(get = "pub")]
        elec_scty_yn: String,
        /// 발행기관코드
        #[getset(get = "pub")]
        issu_istt_cd: String,
        /// ETF추적수익율배수
        #[getset(get = "pub")]
        etf_chas_erng_rt_dbnb: String,
        /// ETFETN투자유의종목여부
        #[getset(get = "pub")]
        etf_etn_ivst_heed_item_yn: String,
        /// 대주이자율구분코드
        #[getset(get = "pub")]
        stln_int_rt_dvsn_cd: String,
        /// 외국인개인한도비율
        #[getset(get = "pub")]
        frnr_psnl_lmt_rt: String,
        /// 상장신청인발행기관코드
        #[getset(get = "pub")]
        lstg_rqsr_issu_istt_cd: String,
        /// 상장신청인종목코드
        #[getset(get = "pub")]
        lstg_rqsr_item_cd: String,
        /// 신탁기관발생기관코드
        #[getset(get = "pub")]
        trst_istt_issu_istt_cd: String,
    }
}

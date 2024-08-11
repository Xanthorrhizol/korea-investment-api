use getset::Getters;
use serde::Deserialize;

pub type DailyPriceResponse = QuoteResponse<Vec<output::DailyPrice>, (), ()>;
pub type VolumeRankResponse = QuoteResponse<Vec<output::VolumeRank>, (), ()>;
pub type GroupListResponse = QuoteResponse<Vec<output::GroupList>, (), Vec<output::GroupList>>;
pub type GroupItemResponse = QuoteResponse<(), output::GroupInfo, Vec<output::GroupItem>>;
pub type BasicStockInfoResponse = QuoteResponse<output::BasicStockInfo, (), ()>;

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
    use crate::types::{
        ExCode, ExchangeCode, MarketId, SecurityGroupId, StockKindCode, VsPriceSign,
    };
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

    #[derive(Clone, Debug, Deserialize, Getters)]
    pub struct BasicStockInfo {
        #[getset(get = "pub")]
        ///상품번호
        pdno: String,
        #[getset(get = "pub")]
        /// 상품유형코드
        prdt_type_cd: String,
        #[getset(get = "pub")]
        /// 시장ID코드
        mket_id_cd: MarketId,
        #[getset(get = "pub")]
        /// 증권그룹ID코드
        scty_grp_id_cd: SecurityGroupId,
        #[getset(get = "pub")]
        /// 거래소구분코드
        excg_dvsn_cd: ExchangeCode,
        #[getset(get = "pub")]
        /// 결산월일
        setl_mmdd: String,
        #[getset(get = "pub")]
        /// 상장주수
        lstg_stqty: Option<String>,
        #[getset(get = "pub")]
        /// 상장자본금액
        lstg_cptl_amt: String,
        #[getset(get = "pub")]
        /// 자본금
        cpta: String,
        #[getset(get = "pub")]
        /// 액면가
        papr: String,
        #[getset(get = "pub")]
        /// 발행가격
        issu_pric: String,
        #[getset(get = "pub")]
        /// 코스피200종목여부
        kospi200_item_yn: String,
        #[getset(get = "pub")]
        /// 유가증권시장상장일자(YYYYmmdd)
        scts_mket_lstg_dt: String,
        #[getset(get = "pub")]
        /// 유가증권시장상장폐지일자(YYYYmmdd)
        scts_mket_lstg_abol_dt: String,
        #[getset(get = "pub")]
        /// 코스닥시장상장일자(YYYYmmdd)
        kosdaq_mket_lstg_dt: String,
        #[getset(get = "pub")]
        /// 코스닥시장상장폐지일자(YYYYmmdd)
        kosdaq_mket_lstg_abol_dt: String,
        #[getset(get = "pub")]
        /// 프리보드시장상장일자(YYYYmmdd)
        frbd_mket_lstg_dt: String,
        #[getset(get = "pub")]
        /// 프리보드시장상장폐지일자(YYYYmmdd)
        frbd_mket_lstg_abol_dt: String,
        #[getset(get = "pub")]
        /// 리츠종류코드
        reits_kind_cd: String,
        #[getset(get = "pub")]
        /// ETF구분코드
        etf_dvsn_cd: String,
        #[getset(get = "pub")]
        /// 유전펀드여부
        oilf_fund_yn: String,
        #[getset(get = "pub")]
        /// 지수업종대분류코드
        idx_bztp_lcls_cd: String,
        #[getset(get = "pub")]
        /// 지수업종중분류코드
        idx_bztp_mcls_cd: String,
        #[getset(get = "pub")]
        /// 지수업종소분류코드
        idx_bztp_scls_cd: String,
        #[getset(get = "pub")]
        /// 주식종류코드
        stck_kind_cd: StockKindCode,
        #[getset(get = "pub")]
        /// 뮤추얼펀드개시일자(YYYYmmdd)
        mfnd_opng_dt: String,
        #[getset(get = "pub")]
        /// 뮤추얼펀드종료일자(YYYYmmdd)
        mfnd_end_dt: String,
        #[getset(get = "pub")]
        /// 예탁등록취소일자(YYYYmmdd)
        dpsi_erlm_cncl_dt: String,
        #[getset(get = "pub")]
        /// ETFCU수량
        etf_cu_qty: String,
        #[getset(get = "pub")]
        /// 상품명
        prdt_name: String,
        #[getset(get = "pub")]
        /// 상품명120
        prdt_name120: String,
        #[getset(get = "pub")]
        /// 상품약어명
        prdt_abrv_name: String,
        #[getset(get = "pub")]
        /// 표준상품번호(ISIN)
        std_pdno: String,
        #[getset(get = "pub")]
        /// 상품영문명
        prdt_eng_name: String,
        #[getset(get = "pub")]
        /// 상품영문명120
        prdt_eng_name120: String,
        #[getset(get = "pub")]
        /// 상품영문약어명
        prdt_eng_abrv_name: String,
        #[getset(get = "pub")]
        /// 예탁지정등록여부
        dpsi_aptm_erlm_yn: String,
        #[getset(get = "pub")]
        /// ETF과세유형코드
        etf_txtn_type_cd: String,
        #[getset(get = "pub")]
        /// ETF유형코드
        etf_type_cd: String,
        #[getset(get = "pub")]
        /// 상장폐지일자(YYYYmmdd)
        lstg_abol_dt: String,
        #[getset(get = "pub")]
        /// 신주구주구분코드
        nwst_odst_dvsn_cd: String,
        #[getset(get = "pub")]
        /// 대용가격
        sbst_pric: String,
        #[getset(get = "pub")]
        /// 당사대용가격
        thco_sbst_pric: String,
        #[getset(get = "pub")]
        /// 당사대용가격변경일자(YYYYmmdd)
        thco_sbst_pric_chng_dt: String,
        #[getset(get = "pub")]
        /// 거래정지여부
        tr_stop_yn: String,
        #[getset(get = "pub")]
        /// 관리종목여부
        admn_item_yn: String,
        #[getset(get = "pub")]
        /// 당일종가
        thdt_clpr: String,
        #[getset(get = "pub")]
        /// 전일종가
        bfdy_clpr: String,
        #[getset(get = "pub")]
        /// 종가변경일자(YYYYmmdd)
        clpr_chng_dt: String,
        #[getset(get = "pub")]
        /// 표준산업분류코드
        std_idst_clsf_cd: String,
        #[getset(get = "pub")]
        /// 표준산업분류코드명
        std_idst_clsf_cd_name: String,
        #[getset(get = "pub")]
        /// 지수업종대분류코드명
        idx_bztp_lcls_cd_name: String,
        #[getset(get = "pub")]
        /// 지수업종중분류코드명
        idx_bztp_mcls_cd_name: String,
        #[getset(get = "pub")]
        /// 지수업종소분류코드명
        idx_bztp_scls_cd_name: String,
        #[getset(get = "pub")]
        /// OCR번호
        ocr_no: String,
        #[getset(get = "pub")]
        /// 크라우드펀딩종목여부
        crfd_item_yn: String,
        #[getset(get = "pub")]
        /// 전자증권여부
        elec_scty_yn: String,
        #[getset(get = "pub")]
        /// 발행기관코드
        issu_istt_cd: String,
        #[getset(get = "pub")]
        /// ETF추적수익율배수
        etf_chas_erng_rt_dbnb: String,
        #[getset(get = "pub")]
        /// ETFETN투자유의종목여부
        etf_etn_ivst_heed_item_yn: String,
        #[getset(get = "pub")]
        /// 대주이자율구분코드
        stln_int_rt_dvsn_cd: String,
        #[getset(get = "pub")]
        /// 외국인개인한도비율
        frnr_psnl_lmt_rt: String,
        #[getset(get = "pub")]
        /// 상장신청인발행기관코드
        lstg_rqsr_issu_istt_cd: String,
        #[getset(get = "pub")]
        /// 상장신청인종목코드
        lstg_rqsr_item_cd: String,
        #[getset(get = "pub")]
        /// 신탁기관발생기관코드
        trst_istt_issu_istt_cd: String,
    }
}

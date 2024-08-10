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
        /// AGR.농축산물파생
        /// BON.채권파생
        /// CMD.일반상품시장
        /// CUR.통화파생
        /// ENG.에너지파생
        /// EQU.주식파생
        /// ETF.ETF파생
        /// IRT.금리파생
        /// KNX.코넥스
        /// KSQ.코스닥
        /// MTL.금속파생
        /// SPI.주가지수파생
        /// STK.유가증권
        mket_id_cd: String,
        #[getset(get = "pub")]
        /// 증권그룹ID코드
        /// BC.수익증권
        /// DR.주식예탁증서
        /// EF.ETF
        /// EN.ETN
        /// EW.ELW
        /// FE.해외ETF
        /// FO.선물옵션
        /// FS.외국주권
        /// FU.선물
        /// FX.플렉스 선물
        /// GD.금현물
        /// IC.투자계약증권
        /// IF.사회간접자본투융자회사
        /// KN.코넥스주권
        /// MF.투자회사
        /// OP.옵션
        /// RT.부동산투자회사
        /// SC.선박투자회사
        /// SR.신주인수권증서
        /// ST.주권
        /// SW.신주인수권증권
        /// TC.신탁수익증권
        scty_grp_id_cd: String,
        #[getset(get = "pub")]
        /// 거래소구분코드
        /// 01.한국증권
        /// 02.증권거래소
        /// 03.코스닥
        /// 04.K-OTC
        /// 05.선물거래소
        /// 06.CME
        /// 07.EUREX
        /// 21.금현물
        /// 50.미국주간
        /// 51.홍콩
        /// 52.상해B
        /// 53.심천
        /// 54.홍콩거래소
        /// 55.미국
        /// 56.일본
        /// 57.상해A
        /// 58.심천A
        /// 59.베트남
        /// 61.장전시간외시장
        /// 64.경쟁대량매매
        /// 65.경매매시장
        /// 81.시간외단일가시장
        excg_dvsn_cd: String,
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
        /// 000.해당사항없음
        /// 101.보통주
        /// 201.우선주
        /// 202.2우선주
        /// 203.3우선주
        /// 204.4우선주
        /// 205.5우선주
        /// 206.6우선주
        /// 207.7우선주
        /// 208.8우선주
        /// 209.9우선주
        /// 210.10우선주
        /// 211.11우선주
        /// 212.12우선주
        /// 213.13우선주
        /// 214.14우선주
        /// 215.15우선주
        /// 216.16우선주
        /// 217.17우선주
        /// 218.18우선주
        /// 219.19우선주
        /// 220.20우선주
        /// 301.후배주
        /// 401.혼합주
        stck_kind_cd: String,
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

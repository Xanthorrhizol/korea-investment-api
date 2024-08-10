use crate::types::{
    BelongClassCode, CustomerType, MarketCode, PeriodCode, Price, ShareClassCode, TargetClassCode,
    TargetExeceptClassCode, TrId,
};
use getset::{CopyGetters, Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Header {
    authorization: String,
    appkey: String,
    appsecret: String,
    tr_id: TrId,
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

#[derive(Debug, Clone, Getters, CopyGetters, Setters, Serialize)]
pub struct DailyPriceParameter {
    #[getset(get = "pub", set = "pub")]
    /// FID 조건 시장 분류 코드
    fid_cond_mrkt_div_code: MarketCode,
    #[getset(get = "pub", set = "pub")]
    /// FID 입력 종목코드
    fid_input_iscd: String,
    #[getset(get = "pub", set = "pub")]
    /// FID 기간 분류 코드
    fid_period_div_code: PeriodCode,
    #[getset(get_copy = "pub", set = "pub")]
    /// FID 수정주가 원주가 가격(수정주가 반영: 0, 수정주가 미반영: 1)
    fid_org_adj_prc: u8,
}
impl DailyPriceParameter {
    pub fn new(
        market_code: MarketCode,
        shortcode: String,
        period_code: PeriodCode,
        is_adjust_price: bool,
    ) -> Self {
        Self {
            fid_cond_mrkt_div_code: market_code,
            fid_input_iscd: shortcode,
            fid_period_div_code: period_code,
            fid_org_adj_prc: match is_adjust_price {
                true => 0,
                false => 1,
            },
        }
    }

    pub fn into_iter(&self) -> [(&'static str, String); 4] {
        [
            (
                "FID_COND_MRKT_DIV_CODE",
                format!("{}", self.fid_cond_mrkt_div_code),
            ),
            ("FID_INPUT_ISCD", self.fid_input_iscd.clone()),
            (
                "FID_PERIOD_DIV_CODE",
                format!("{}", self.fid_period_div_code),
            ),
            ("FID_ORG_ADJ_PRC", format!("{}", self.fid_org_adj_prc)),
        ]
    }
}

#[derive(Debug, Clone, Getters, CopyGetters, Setters, Serialize)]
pub struct VolumeRankParameter {
    #[getset(get = "pub", set = "pub")]
    /// 조건 시장 분류 코드(J)
    fid_cond_mrkt_div_code: MarketCode,
    #[getset(get = "pub", set = "pub")]
    /// 조건 화면 분류 코드(20171)
    fid_cond_scr_div_code: String,
    #[getset(get = "pub", set = "pub")]
    /// 입력 종목코드(전체: 0000, 기타: 업종코드)
    fid_input_iscd: String,
    #[getset(get = "pub", set = "pub")]
    /// 분류구분코드(0: 전체, 1: 보통주, 2: 우선주)
    fid_div_cls_code: ShareClassCode,
    #[getset(get = "pub", set = "pub")]
    /// 소속 구분 코드
    fid_blng_cls_code: BelongClassCode,
    #[getset(get = "pub", set = "pub")]
    /// 대상구분코드(1 or 0 9자리)
    /// 차례대로 증거금 30 40 50 60 100 신용보증금 30 40 50 60
    fid_trgt_cls_code: TargetClassCode,
    #[getset(get = "pub", set = "pub")]
    /// 대상제외구분코드(1 or 0 6자리)
    /// 차례대로 투자위험/경고/주의 관리종목 정리매매 불성실공시 우선주 거래정지
    fid_trgt_exls_cls_code: TargetExeceptClassCode,
    #[getset(get_copy = "pub", set = "pub")]
    /// 입력 가격1(최소)
    fid_input_price_1: Option<Price>,
    #[getset(get_copy = "pub", set = "pub")]
    /// 입력 가격2(최대)
    fid_input_price_2: Option<Price>,
    #[getset(get_copy = "pub", set = "pub")]
    /// 거래량 수
    fid_vol_cnt: Option<u32>,
    #[getset(get = "pub", set = "pub")]
    /// 입력 날짜1("")
    fid_input_date_1: String,
}

impl VolumeRankParameter {
    pub fn new(
        fid_input_iscd: String,           // 입력 종목코드(전체: 0000, 기타: 업종코드)
        fid_div_cls_code: ShareClassCode, // 분류구분코드(0: 전체, 1: 보통주, 2: 우선주)
        fid_blng_cls_code: BelongClassCode, // 소속 구분 코드
        fid_trgt_cls_code: TargetClassCode, // 대상구분코드(1 or 0 9자리)
        fid_trgt_exls_cls_code: TargetExeceptClassCode, // 대상제외구분코드(1 or 0 6자리)
        fid_input_price_1: Option<Price>, // 입력 가격1(최소)
        fid_input_price_2: Option<Price>, // 입력 가격2(최대)
        fid_vol_cnt: Option<u32>,         // 거래량 수
    ) -> Self {
        Self {
            fid_cond_mrkt_div_code: MarketCode::Stock,
            fid_cond_scr_div_code: "20171".to_string(),
            fid_input_iscd,
            fid_div_cls_code,
            fid_blng_cls_code,
            fid_trgt_cls_code,
            fid_trgt_exls_cls_code,
            fid_input_price_1,
            fid_input_price_2,
            fid_vol_cnt,
            fid_input_date_1: "".to_string(),
        }
    }

    pub fn into_iter(&self) -> [(&'static str, String); 11] {
        [
            (
                "FID_COND_MRKT_DIV_CODE",
                format!("{}", self.fid_cond_mrkt_div_code),
            ),
            (
                "FID_COND_SCR_DIV_CODE",
                format!("{}", self.fid_cond_scr_div_code),
            ),
            ("FID_INPUT_ISCD", format!("{}", self.fid_input_iscd)),
            ("FID_DIV_CLS_CODE", format!("{}", self.fid_div_cls_code)),
            ("FID_BLNG_CLS_CODE", format!("{}", self.fid_blng_cls_code)),
            ("FID_TRGT_CLS_CODE", format!("{}", self.fid_trgt_cls_code)),
            (
                "FID_TRGT_EXLS_CLS_CODE",
                format!("{}", self.fid_trgt_exls_cls_code),
            ),
            (
                "FID_INPUT_PRICE_1",
                if let Some(price) = self.fid_input_price_1 {
                    format!("{}", price.inner)
                } else {
                    "".to_string()
                },
            ),
            (
                "FID_INPUT_PRICE_2",
                if let Some(price) = self.fid_input_price_2 {
                    format!("{}", price.inner)
                } else {
                    "".to_string()
                },
            ),
            (
                "FID_VOL_CNT",
                if let Some(count) = self.fid_vol_cnt {
                    count.to_string()
                } else {
                    "".to_string()
                },
            ),
            ("FID_INPUT_DATE_1", format!("{}", self.fid_input_date_1)),
        ]
    }
}

#[derive(Debug, Clone, Getters, CopyGetters, Setters, Serialize)]
pub struct GroupListParameter {
    #[getset(get = "pub", set = "pub")]
    type_: String,
    #[getset(get = "pub", set = "pub")]
    fid_etc_cls_code: String,
    #[getset(get = "pub", set = "pub")]
    user_id: String,
}

impl GroupListParameter {
    pub fn new(user_id: &str) -> Self {
        let user_id = user_id.to_string();
        Self {
            type_: "1".to_string(),
            fid_etc_cls_code: "00".to_string(),
            user_id,
        }
    }

    pub fn into_iter(&self) -> [(&'static str, String); 3] {
        [
            ("TYPE", self.type_.clone()),
            ("FID_ETC_CLS_CODE", self.fid_etc_cls_code.clone()),
            ("USER_ID", self.user_id.clone()),
        ]
    }
}

#[derive(Debug, Clone, Getters, CopyGetters, Setters, Serialize)]
pub struct GroupItemParameter {
    #[getset(get = "pub", set = "pub")]
    type_: String,
    #[getset(get = "pub", set = "pub")]
    user_id: String,
    #[getset(get = "pub", set = "pub")]
    data_rank: String,
    #[getset(get = "pub", set = "pub")]
    inter_grp_code: String,
    #[getset(get = "pub", set = "pub")]
    inter_grp_name: String,
    #[getset(get = "pub", set = "pub")]
    hts_kor_isnm: String,
    #[getset(get = "pub", set = "pub")]
    cntg_cls_code: String,
    #[getset(get = "pub", set = "pub")]
    fid_etc_cls_code: String,
}

impl GroupItemParameter {
    pub fn new(user_id: &str, inter_grp_code: &str) -> Self {
        let user_id = user_id.to_string();
        let data_rank = "".to_string();
        let inter_grp_code = inter_grp_code.to_string();
        let inter_grp_name = "".to_string();
        let hts_kor_isnm = "".to_string();
        let cntg_cls_code = "".to_string();
        Self {
            type_: "1".to_string(),
            user_id,
            data_rank,
            inter_grp_code,
            inter_grp_name,
            hts_kor_isnm,
            cntg_cls_code,
            fid_etc_cls_code: "4".to_string(),
        }
    }

    pub fn into_iter(&self) -> [(&'static str, String); 8] {
        [
            ("TYPE", self.type_.clone()),
            ("USER_ID", self.user_id.clone()),
            ("DATA_RANK", self.data_rank.clone()),
            ("INTER_GRP_CODE", self.inter_grp_code.clone()),
            ("INTER_GRP_NAME", self.inter_grp_name.clone()),
            ("HTS_KOR_ISNM", self.hts_kor_isnm.clone()),
            ("CNTG_CLS_CODE", self.cntg_cls_code.clone()),
            ("FID_ETC_CLS_CODE", self.fid_etc_cls_code.clone()),
        ]
    }
}

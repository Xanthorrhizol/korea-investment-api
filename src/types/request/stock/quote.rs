use crate::types::{
    BelongClassCode, CustomerType, MarketCode, PeriodCode, Price, ShareClassCode, TargetClassCode,
    TargetExeceptClassCode, TrId,
};
use getset::{CopyGetters, Getters};
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

#[derive(Getters, CopyGetters, Serialize)]
pub struct DailyPriceParameter {
    #[getset(get = "pub")]
    fid_cond_mrkt_div_code: MarketCode, // FID 조건 시장 분류 코드
    #[getset(get = "pub")]
    fid_input_iscd: String, // FID 입력 종목코드
    #[getset(get = "pub")]
    fid_period_div_code: PeriodCode, // FID 기간 분류 코드
    #[getset(get_copy = "pub")]
    fid_org_adj_prc: u8, // FID 수정주가 원주가 가격(수정주가 반영: 0, 수정주가 미반영: 1)
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

#[derive(Getters, CopyGetters, Serialize)]
pub struct VolumeRankParameter {
    #[getset(get = "pub")]
    fid_cond_mrkt_div_code: MarketCode, // 조건 시장 분류 코드(J)
    #[getset(get = "pub")]
    fid_cond_scr_div_code: String, // 조건 화면 분류 코드(20171)
    #[getset(get = "pub")]
    fid_input_iscd: String, // 입력 종목코드(전체: 0000, 기타: 업종코드)
    #[getset(get = "pub")]
    fid_div_cls_code: ShareClassCode, // 분류구분코드(0: 전체, 1: 보통주, 2: 우선주)
    #[getset(get = "pub")]
    fid_blng_cls_code: BelongClassCode, // 소속 구분 코드
    #[getset(get = "pub")]
    fid_trgt_cls_code: TargetClassCode, // 대상구분코드(1 or 0 9자리)
    // 차례대로 증거금 30 40 50 60 100 신용보증금 30 40 50 60
    #[getset(get = "pub")]
    fid_trgt_exls_cls_code: TargetExeceptClassCode, // 대상제외구분코드(1 or 0 6자리)
    // 차례대로 투자위험/경고/주의 관리종목 정리매매 불성실공시 우선주 거래정지
    #[getset(get_copy = "pub")]
    fid_input_price_1: Option<Price>, // 입력 가격1(최소)
    #[getset(get_copy = "pub")]
    fid_input_price_2: Option<Price>, // 입력 가격2(최대)
    #[getset(get_copy = "pub")]
    fid_vol_cnt: Option<u32>, // 거래량 수
    #[getset(get = "pub")]
    fid_input_date_1: String, // 입력 날짜1("")
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
                format!(
                    "{}",
                    if let Some(price) = self.fid_input_price_1 {
                        price.inner
                    } else {
                        0
                    }
                ),
            ),
            (
                "FID_INPUT_PRICE_2",
                format!(
                    "{}",
                    if let Some(price) = self.fid_input_price_2 {
                        price.inner
                    } else {
                        0
                    }
                ),
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

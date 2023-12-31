use crate::types::{CustomerType, MarketCode, PeriodCode, TrId};
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

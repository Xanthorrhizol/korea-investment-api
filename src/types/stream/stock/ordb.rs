use super::{Header, StreamParser};
use crate::types::{DealClassCode, Time, TimeClassCode, VsPriceSign};
use crate::util::get_json_inner;
use crate::Error;

#[derive(Debug, Clone)]
pub struct Ordb {
    header: Header,
    body: Option<Body>,
}

unsafe impl Send for Ordb {}

impl StreamParser<Body> for Ordb {
    fn parse(s: String) -> Result<Self, Error> {
        if let Ok(j) = json::parse(&s) {
            let header = Header {
                tr_id: get_json_inner(&j, "header.tr_id")?.as_str().unwrap().into(),
                datetime: Time::parse(
                    get_json_inner(&j, "header.datetime")?.as_str().unwrap(),
                    "%Y%m%d%H%M%S",
                )?,
            };
            Ok(Self { header, body: None })
        } else {
            let splits = s.split('^').collect::<Vec<&str>>();
            let business_operation_date = chrono::Utc::now()
                .with_timezone(&chrono_tz::Asia::Seoul)
                .format("%Y%m%d")
                .to_string();
            let header_str = splits[0].split('|').collect::<Vec<&str>>();
            let encrypted = header_str[0] == "1";
            let time = Time::parse(&(business_operation_date + splits[1]), "%Y%m%d%H%M%S")?;
            let header = Header {
                tr_id: header_str[1].into(),
                datetime: time.clone(),
            };
            let body = if encrypted {
                None // TODO
            } else {
                let ask_price = {
                    let mut result = [0u32; 10];
                    for i in 3..13 {
                        result[i - 3] = splits[i].parse()?;
                    }
                    result
                };
                let bid_price = {
                    let mut result = [0u32; 10];
                    for i in 13..23 {
                        result[i - 13] = splits[i].parse()?;
                    }
                    result
                };
                let ask_remained = {
                    let mut result = [0u64; 10];
                    for i in 23..33 {
                        result[i - 23] = splits[i].parse()?;
                    }
                    result
                };
                let bid_remained = {
                    let mut result = [0u64; 10];
                    for i in 33..43 {
                        result[i - 33] = splits[i].parse()?;
                    }
                    result
                };
                Some(Body {
                    shortcode: header_str[3].to_string(),
                    time,
                    time_class_code: splits[2].into(),
                    ask_price,
                    bid_price,
                    ask_remained,
                    bid_remained,
                    total_ask_order_remained: splits[43].parse()?,
                    total_bid_order_remained: splits[44].parse()?,
                    total_otc_ask_order_remained: splits[45].parse()?,
                    total_otc_bid_order_remained: splits[46].parse()?,
                    predicted_exec_price: splits[47].parse()?,
                    predicted_exec_quantity: splits[48].parse()?,
                    predicted_volume: splits[49].parse()?,
                    predicted_vs_exec: splits[50].parse()?,
                    predicted_vs_exec_sign: splits[51].into(),
                    predicted_exec_price_rate_vs_yesterday: splits[52].parse()?,
                    accumulative_exec_volume: splits[53].parse()?,
                    total_ask_order_remained_diff: splits[54].parse()?,
                    total_bid_order_remained_diff: splits[55].parse()?,
                    total_otc_ask_order_remained_diff: splits[56].parse()?,
                    total_otc_bid_order_remained_diff: splits[57].parse()?,
                    stock_deal_class_code: splits[58].into(),
                })
            };
            Ok(Self { header, body })
        }
    }

    fn header(&self) -> &Header {
        &self.header
    }

    fn body(&self) -> &Option<Body> {
        &self.body
    }
}

#[derive(Debug, Clone)]
pub struct Body {
    pub shortcode: String,                 // MKSC_SHRN_ISCD(유가증권 단축 종목코드)
    pub time: Time,                        // BSOP_HOUR(영업 시간)
    pub time_class_code: TimeClassCode,    // HOUR_CLS_CODE(시간 구분 코드)
    pub ask_price: [u32; 10],              // ASKP1~10(매도호가)
    pub bid_price: [u32; 10],              // BIDP1~10(매수호가)
    pub ask_remained: [u64; 10],           // ASKP_RSQN1~10(매도호가 잔량)
    pub bid_remained: [u64; 10],           // BIDP_RSQN1~10(매수호가 잔량)
    pub total_ask_order_remained: u64,     // TOTAL_ASKP_RSQN(총 매도호가 잔량)
    pub total_bid_order_remained: u64,     // TOTAL_BIDP_RSQN(총 매수호가 잔량)
    pub total_otc_ask_order_remained: u64, // OVTM_TOTAL_ASKP_RSQN(시간외 총 매도호가 잔량)
    pub total_otc_bid_order_remained: u64, // OVTM_TOTAL_ASKP_RSQN(시간외 총 매수호가 잔량)
    pub predicted_exec_price: u32,         // ANTC_CNPR(예상 체결가)
    pub predicted_exec_quantity: u64,      // ANTC_CNQN(예상 체결량)
    pub predicted_volume: u64,             // ANTC_VOL(예상 거래량)
    pub predicted_vs_exec: i32,            // ANTC_CNTG_VRSS(예상 체결 대비)
    pub predicted_vs_exec_sign: VsPriceSign, // ANTC_CNTG_VRSS_SIGN(예상 체결 대비 부호)
    pub predicted_exec_price_rate_vs_yesterday: f32, // ANTC_CNTG_PRDY_CTRT(예상 체결 전일 대비율)
    pub accumulative_exec_volume: u64,     // ACML_VOL(누적 거래량) - Exture 3.0에서 제거됨 -> 0
    pub total_ask_order_remained_diff: i64, // TOTAL_ASKP_RSQN_ICDC(총 매도호가 잔량 증감)
    pub total_bid_order_remained_diff: i64, // TOTAL_BIDP_RSQN_ICDC(총 매수호가 잔량 증감)
    pub total_otc_ask_order_remained_diff: i64, // OVTM_TOTAL_ASKP_RSQN_ICDC(시간외 총 매도호가 잔량 증감)
    pub total_otc_bid_order_remained_diff: i64, // OVTM_TOTAL_BIDP_RSQN_ICDC(시간외 총 매수호가 잔량 증감)
    pub stock_deal_class_code: DealClassCode,   // STCK_DEAL_CLS_CODE(주식 매매 구분 코드)
}

use super::{Header, StreamParser};
use crate::Error;
use crate::types::{DealClassCode, Time, TimeClassCode, VsPriceSign};
use crate::util::get_json_inner;

#[derive(Debug, Clone)]
pub struct Ordb {
    header: Header,
    body: Vec<Body>,
}

unsafe impl Send for Ordb {}

impl StreamParser<Body> for Ordb {
    fn parse(s: String) -> Result<Self, Error> {
        if let Ok(j) = json::parse(&s) {
            let header = Header {
                tr_id: get_json_inner(&j, "header.tr_id")?
                    .as_str()
                    .ok_or(crate::Error::InvalidData)?
                    .parse()?,
                datetime: Time::parse(
                    get_json_inner(&j, "header.datetime")?.as_str().unwrap(),
                    "%Y%m%d%H%M%S",
                )?,
            };
            Ok(Self {
                header,
                body: Vec::new(),
            })
        } else {
            let splits = s.split('^').collect::<Vec<&str>>();
            let business_operation_date = chrono::Utc::now()
                .with_timezone(&chrono_tz::Asia::Seoul)
                .format("%Y%m%d")
                .to_string();
            let header_str = splits[0].split('|').collect::<Vec<&str>>();
            let encrypted = header_str[0] == "1";
            let time = Time::parse(
                &(business_operation_date.to_string() + splits[1]),
                "%Y%m%d%H%M%S",
            )?;
            let tr_id = header_str[1].parse()?;
            let column_count = Self::get_column_count(&tr_id);
            let header = Header {
                tr_id,
                datetime: time.clone(),
            };
            let body = if encrypted {
                Vec::new() // TODO
            } else {
                if let Ok(count) = header_str[2].parse() {
                    let mut bodies = Vec::with_capacity(count);
                    for i in 0..count {
                        let time = Time::parse(
                            &(business_operation_date.to_string() + splits[i * column_count + 1]),
                            "%Y%m%d%H%M%S",
                        )?;
                        let ask_price = {
                            let mut result = [0u32; 10];
                            for j in 3..13 {
                                result[j - 3] = splits[i * column_count + j].parse()?;
                            }
                            result
                        };
                        let bid_price = {
                            let mut result = [0u32; 10];
                            for j in 13..23 {
                                result[j - 13] = splits[i * column_count + j].parse()?;
                            }
                            result
                        };
                        let ask_remained = {
                            let mut result = [0u64; 10];
                            for j in 23..33 {
                                result[j - 23] = splits[i * column_count + j].parse()?;
                            }
                            result
                        };
                        let bid_remained = {
                            let mut result = [0u64; 10];
                            for j in 33..43 {
                                result[j - 33] = splits[i * column_count + j].parse()?;
                            }
                            result
                        };
                        bodies.push(Body {
                            shortcode: header_str[3].to_string(),
                            time,
                            time_class_code: splits[i * column_count + 2].into(),
                            ask_price,
                            bid_price,
                            ask_remained,
                            bid_remained,
                            total_ask_order_remained: splits[i * column_count + 43].parse()?,
                            total_bid_order_remained: splits[i * column_count + 44].parse()?,
                            total_otc_ask_order_remained: splits[i * column_count + 45].parse()?,
                            total_otc_bid_order_remained: splits[i * column_count + 46].parse()?,
                            predicted_exec_price: splits[i * column_count + 47].parse()?,
                            predicted_exec_quantity: splits[i * column_count + 48].parse()?,
                            predicted_volume: splits[i * column_count + 49].parse()?,
                            predicted_vs_exec: splits[i * column_count + 50].parse()?,
                            predicted_vs_exec_sign: splits[i * column_count + 51].into(),
                            predicted_exec_price_rate_vs_yesterday: splits[i * column_count + 52]
                                .parse()?,
                            accumulative_exec_volume: splits[i * column_count + 53].parse()?,
                            total_ask_order_remained_diff: splits[i * column_count + 54].parse()?,
                            total_bid_order_remained_diff: splits[i * column_count + 55].parse()?,
                            total_otc_ask_order_remained_diff: splits[i * column_count + 56]
                                .parse()?,
                            total_otc_bid_order_remained_diff: splits[i * column_count + 57]
                                .parse()?,
                            stock_deal_class_code: splits[i * column_count + 58].into(),
                        });
                    }
                    bodies
                } else {
                    Vec::new()
                }
            };
            Ok(Self { header, body })
        }
    }

    fn header(&self) -> &Header {
        &self.header
    }

    fn body(&self) -> &Vec<Body> {
        &self.body
    }
}

#[derive(Debug, Clone)]
pub struct Body {
    /// MKSC_SHRN_ISCD(유가증권 단축 종목코드)
    pub shortcode: String,
    /// BSOP_HOUR(영업 시간)
    pub time: Time,
    /// HOUR_CLS_CODE(시간 구분 코드)
    pub time_class_code: TimeClassCode,
    /// ASKP1~10(매도호가)
    pub ask_price: [u32; 10],
    /// BIDP1~10(매수호가)
    pub bid_price: [u32; 10],
    /// ASKP_RSQN1~10(매도호가 잔량)
    pub ask_remained: [u64; 10],
    /// BIDP_RSQN1~10(매수호가 잔량)
    pub bid_remained: [u64; 10],
    /// TOTAL_ASKP_RSQN(총 매도호가 잔량)
    pub total_ask_order_remained: u64,
    /// TOTAL_BIDP_RSQN(총 매수호가 잔량)
    pub total_bid_order_remained: u64,
    /// OVTM_TOTAL_ASKP_RSQN(시간외 총 매도호가 잔량)
    pub total_otc_ask_order_remained: u64,
    /// OVTM_TOTAL_ASKP_RSQN(시간외 총 매수호가 잔량)
    pub total_otc_bid_order_remained: u64,
    /// ANTC_CNPR(예상 체결가)
    pub predicted_exec_price: u32,
    /// ANTC_CNQN(예상 체결량)
    pub predicted_exec_quantity: u64,
    /// ANTC_VOL(예상 거래량)
    pub predicted_volume: u64,
    /// ANTC_CNTG_VRSS(예상 체결 대비)
    pub predicted_vs_exec: i32,
    /// ANTC_CNTG_VRSS_SIGN(예상 체결 대비 부호)
    pub predicted_vs_exec_sign: VsPriceSign,
    /// ANTC_CNTG_PRDY_CTRT(예상 체결 전일 대비율)
    pub predicted_exec_price_rate_vs_yesterday: f32,
    /// ACML_VOL(누적 거래량) - Exture 3.0에서 제거됨 -> 0
    pub accumulative_exec_volume: u64,
    /// TOTAL_ASKP_RSQN_ICDC(총 매도호가 잔량 증감)
    pub total_ask_order_remained_diff: i64,
    /// TOTAL_BIDP_RSQN_ICDC(총 매수호가 잔량 증감)
    pub total_bid_order_remained_diff: i64,
    /// OVTM_TOTAL_ASKP_RSQN_ICDC(시간외 총 매도호가 잔량 증감)
    pub total_otc_ask_order_remained_diff: i64,
    /// OVTM_TOTAL_BIDP_RSQN_ICDC(시간외 총 매수호가 잔량 증감)
    pub total_otc_bid_order_remained_diff: i64,
    /// STCK_DEAL_CLS_CODE(주식 매매 구분 코드)
    pub stock_deal_class_code: DealClassCode,
}

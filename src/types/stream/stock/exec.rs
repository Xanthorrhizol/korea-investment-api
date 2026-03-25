use super::{Header, StreamParser};
use crate::Error;
use crate::types::{
    ExecClass, MarketOperationClassCode, MarketTerminationClassCode, Time, TimeClassCode,
    VsPriceSign, parse_bool,
};
use crate::util::get_json_inner;

#[derive(Debug, Clone)]
pub struct Exec {
    header: Header,
    body: Vec<Body>,
}

unsafe impl Send for Exec {}

impl StreamParser<Body> for Exec {
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
            let business_operation_date = splits[33];
            let header_str = splits[0].split('|').collect::<Vec<&str>>();
            let encrypted = header_str[0] == "1";
            let exec_time = Time::parse(
                &(business_operation_date.to_string() + splits[1]),
                "%Y%m%d%H%M%S",
            )?;
            let tr_id = header_str[1].parse()?;

            let column_count = Self::get_column_count(&tr_id);
            let header = Header {
                tr_id,
                datetime: exec_time.clone(),
            };
            let body = if encrypted {
                Vec::new() // TODO
            } else {
                if let Ok(count) = header_str[2].parse() {
                    let mut bodies = Vec::with_capacity(count);
                    for i in 0..count {
                        let exec_time = Time::parse(
                            &(business_operation_date.to_string() + splits[i * column_count + 1]),
                            "%Y%m%d%H%M%S",
                        )?;
                        bodies.push(Body {
                            shortcode: header_str[3].to_string(),
                            exec_time,
                            current_price: splits[i * column_count + 2].parse()?,
                            price_sign_vs_yesterday: splits[i * column_count + 3].into(),
                            price_vs_yesterday: splits[i * column_count + 4].parse()?,
                            price_rate_vs_yesterday: splits[i * column_count + 5].parse()?,
                            weighted_average_price: splits[i * column_count + 6].parse()?,
                            market_price: splits[i * column_count + 7].parse()?,
                            market_upper_price: splits[i * column_count + 8].parse()?,
                            market_lower_price: splits[i * column_count + 9].parse()?,
                            ask_price: splits[i * column_count + 10].parse()?,
                            bid_price: splits[i * column_count + 11].parse()?,
                            exec_volume: splits[i * column_count + 12].parse()?,
                            accumulative_exec_volume: splits[i * column_count + 13].parse()?,
                            accumulative_exec_amount: splits[i * column_count + 14].parse()?,
                            ask_exec_count: splits[i * column_count + 15].parse()?,
                            bid_exec_count: splits[i * column_count + 16].parse()?,
                            natural_bid_exec_count: splits[i * column_count + 17].parse()?,
                            volume_power: splits[i * column_count + 18].parse()?,
                            total_ask_exec_volume: splits[i * column_count + 19].parse()?,
                            total_bid_exec_volume: splits[i * column_count + 20].parse()?,
                            exec_class: splits[i * column_count + 21].into(),
                            bid_rate: splits[i * column_count + 22].parse()?,
                            exec_volume_rate_vs_yesterday: splits[i * column_count + 23].parse()?,
                            market_price_time: Time::parse(
                                &(business_operation_date.to_string()
                                    + splits[i * column_count + 24]),
                                "%Y%m%d%H%M%S",
                            )?,
                            vs_market_price_sign: splits[i * column_count + 25].into(),
                            vs_market_price: splits[i * column_count + 26].parse()?,
                            upper_price_time: Time::parse(
                                &(business_operation_date.to_string()
                                    + splits[i * column_count + 27]),
                                "%Y%m%d%H%M%S",
                            )?,
                            vs_upper_price_sign: splits[i * column_count + 28].into(),
                            vs_upper_price: splits[i * column_count + 29].parse()?,
                            lower_price_time: Time::parse(
                                &(business_operation_date.to_string()
                                    + splits[i * column_count + 30]),
                                "%Y%m%d%H%M%S",
                            )?,
                            vs_lower_price_sign: splits[i * column_count + 31].into(),
                            vs_lower_price: splits[i * column_count + 32].parse()?,
                            business_operation_date: Time::parse(
                                &(splits[i * column_count + 33].to_string() + "000000"),
                                "%Y%m%d%H%M%S",
                            )?,
                            new_market_operation_class_code: splits[i * column_count + 34].into(),
                            trade_suspended: parse_bool(splits[i * column_count + 35]),
                            ask_order_remained: splits[i * column_count + 36].parse()?,
                            bid_order_remained: splits[i * column_count + 37].parse()?,
                            total_ask_order_remained: splits[i * column_count + 38].parse()?,
                            total_bid_order_remained: splits[i * column_count + 39].parse()?,
                            turnover_ratio: splits[i * column_count + 40].parse()?,
                            yesterday_symmetric_time_accumulate_volume: splits
                                [i * column_count + 41]
                                .parse()?,
                            yesterday_symmetric_time_accumulate_volume_rate: splits
                                [i * column_count + 42]
                                .parse()?,
                            time_class_code: splits[i * column_count + 43].into(),
                            market_termination_class_code: splits[i * column_count + 44].into(),
                            vi_standard_price: splits[i * column_count + 45]
                                .parse()
                                .unwrap_or_else(|_| 0),
                        });
                    }
                    bodies
                } else {
                    warn!("invalid data: {}", s);
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
    /// STCK_CNTG_HOUR(주식 체결 시간)
    pub exec_time: Time,
    /// STCK_PRPR(주식 현재가)
    pub current_price: u32,
    /// PRDY_VRSS_SIGN(전일 대비 부호)
    pub price_sign_vs_yesterday: VsPriceSign,
    /// PRDY_VRSS(전일 대비)
    pub price_vs_yesterday: i32,
    /// PRDY_CTRT(전일 대비율)
    pub price_rate_vs_yesterday: f32,
    /// WGHN_AVRG_STCK_PRC(가중 평균 주식 가격)
    pub weighted_average_price: f64,
    /// STCK_OPRC(주식 시가)
    pub market_price: u32,
    /// STCK_HGPR(주식 최고가)
    pub market_upper_price: u32,
    /// STCK_LWPR(주식 최저가)
    pub market_lower_price: u32,
    /// ASK1(매도호가1)
    pub ask_price: u32,
    /// BID1(매수호가1)
    pub bid_price: u32,
    /// CNTG_VOL(체결 거래량)
    pub exec_volume: u64,
    /// ACML_VOL(누적 거래량)
    pub accumulative_exec_volume: u64,
    /// ACML_TR_PBMN(누적 거래 대금)
    pub accumulative_exec_amount: u64,
    /// SELN_CNTG_CSNU(매도 체결 건수)
    pub ask_exec_count: u32,
    /// SHNU_CNTG_CSNU(매수 체결 건수)
    pub bid_exec_count: u32,
    /// NTBY_CNTG_CSNU(순매수 체결 건수)
    pub natural_bid_exec_count: i64,
    /// CTTR(체결강도)
    pub volume_power: f64,
    /// SELN_CNTG_SMTN(총 매도 수량)
    pub total_ask_exec_volume: u64,
    /// SHNU_CNTG_SMTN(총 매수 수량)
    pub total_bid_exec_volume: u64,
    /// CCLD_DVSN(체결구분)
    pub exec_class: ExecClass,
    /// SHNU_RATE(매수비율)
    pub bid_rate: f64,
    /// PRDY_VOL_VRSS_ACML_VOL_RATE(전일 거래량 대비 등락율)
    pub exec_volume_rate_vs_yesterday: f64,
    /// OPRC_HOUR(시가 시간)
    pub market_price_time: Time,
    /// OPRC_VRSS_PRPR_SIGN(시가대비구분)
    pub vs_market_price_sign: VsPriceSign,
    /// OPRC_VRSS_PRPR(시가대비)
    pub vs_market_price: i32,
    /// HGPR_HOUR(최고가 시간)
    pub upper_price_time: Time,
    /// HGPR_VRSS_PRPR_SIGN(고가대비구분)
    pub vs_upper_price_sign: VsPriceSign,
    /// HGPR_VRSS_PRPR(고가대비)
    pub vs_upper_price: i32,
    /// LWPR_HOUR(최저가 시간)
    pub lower_price_time: Time,
    /// LWPR_VRSS_PRPR_SIGN(저가대비구분)
    pub vs_lower_price_sign: VsPriceSign,
    /// LWPR_VRSS_PRPR(저가대비)
    pub vs_lower_price: i32,
    /// BSOP_DATE(영업일자)
    pub business_operation_date: Time,
    /// NEW_MKOP_CLS_CODE(신 장운영 구분 코드)
    pub new_market_operation_class_code: MarketOperationClassCode,
    /// TRHT_YN(거래정지 여부)
    pub trade_suspended: bool,
    /// ASKP_RSQN1(매도호가 잔량1)
    pub ask_order_remained: u64,
    /// BIDP_RSQN1(매수호가 잔량1)
    pub bid_order_remained: u64,
    /// TOTAL_ASKP_RSQN(총 매도호가 잔량)
    pub total_ask_order_remained: u64,
    /// TOTAL_BIDP_RSQN(총 매수호가 잔량)
    pub total_bid_order_remained: u64,
    /// VOL_TNRT(거래량 회전율)
    pub turnover_ratio: f64,
    /// PRDY_SMNS_HOUR_ACML_VOL(전일 동시간 누적 거래량)
    pub yesterday_symmetric_time_accumulate_volume: u64,
    /// PRDY_SMNS_HOUR_ACML_VOL_RATE(전일 동시간 누적 거래량 비율)
    pub yesterday_symmetric_time_accumulate_volume_rate: f64,
    /// HOUR_CLS_CODE(시간 구분 코드)
    pub time_class_code: TimeClassCode,
    /// MRKT_TRTM_CLS_CODE(임의종료구분코드)
    pub market_termination_class_code: MarketTerminationClassCode,
    /// VI_STND_PRC(정적VI발동기준가)
    pub vi_standard_price: u32,
}

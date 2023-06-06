use crate::types::{
    ExecClass, Header, MarketOperationClassCode, MarketTerminationClassCode, Time, TimeClassCode,
    VsPriceSign,
};
use crate::Error;

fn get_json_inner(obj: &json::object::Object, key: &str) -> json::JsonValue {
    if let Some(v) = obj.get(key) {
        v.to_owned()
    } else {
        json::JsonValue::Null
    }
}

#[derive(Debug)]
pub struct Exec {
    header: Header,
    body: Option<Body>,
}

impl Exec {
    pub fn parse(val: json::JsonValue) -> Result<Self, Error> {
        let result = if let json::JsonValue::Object(obj) = val {
            let header = match get_json_inner(&obj, "header") {
                json::JsonValue::Object(o) => Header {
                    tr_id: match get_json_inner(&o, "tr_id") {
                        json::JsonValue::Short(s) => s.to_string().into(),
                        _ => {
                            return Err(Error::InvalidData);
                        }
                    },
                    datetime: match get_json_inner(&o, "datetime") {
                        json::JsonValue::Short(s) => Time::parse(&s.to_string(), "%Y%m%d%H%M%S")?,
                        _ => {
                            return Err(Error::InvalidData);
                        }
                    },
                },
                _ => {
                    return Err(Error::InvalidData);
                }
            };
            let body = match get_json_inner(&obj, "body") {
                json::JsonValue::Null => None,
                json::JsonValue::Object(o) => Some(Body::parse(o)?),
                _ => {
                    return Err(Error::InvalidData);
                }
            };
            Ok(Self { header, body })
        } else {
            return Err(Error::InvalidData);
        };
        result
    }
}

#[derive(Debug)]
pub struct Body {
    shortcode: String,                    // MKSC_SHRN_ISCD(유가증권 단축 종목코드)
    exec_time: Time,                      // STCK_CNTG_HOUR(주식 체결 시간)
    current_price: u32,                   // STCK_PRPR(주식 현재가)
    price_sign_vs_yesterday: VsPriceSign, // PRDY_VRSS_SIGN(전일 대비 부호)
    price_vs_yesterday: u32,              // PRDY_VRSS(전일 대비)
    price_rate_vs_yesterday: f32,         // PRDY_CTRT(전일 대비율)
    weighted_average_price: u64,          // WGHN_AVRG_STCK_PRC(가중 평균 주식 가격)
    market_price: u32,                    // STCK_OPRC(주식 시가)
    market_upper_price: u32,              // STCK_HGPR(주식 최고가)
    market_lower_price: u32,              // STCK_LWPR(주식 최저가)
    ask_price: u32,                       // ASKP1(매도호가1)
    bid_price: u32,                       // BIDP1(매수호가1)
    exec_volume: u64,                     // CNTG_VOL(체결 거래량)
    accumulative_exec_volume: u64,        // ACML_VOL(누적 거래량)
    accumulative_exec_amount: u64,        // ACML_TR_PBMN(누적 거래 대금)
    ask_exec_count: u32,                  // SELN_CNTG_CSNU(매도 체결 건수)
    bid_exec_count: u32,                  // SHNU_CNTG_CSNU(매수 체결 건수)
    natural_bid_exec_count: u32,          // NTBY_CNTG_CSNU(순매수 체결 건수)
    volume_power: u64,                    // CTTR(체결강도)
    total_ask_exec_volume: u64,           // SELN_CNTG_SMTN(총 매도 수량)
    total_bid_exec_volume: u64,           // SHNU_CNTG_SMTN(총 매수 수량)
    exec_class: ExecClass,                // CCLD_DVSN(체결구분)
    bid_rate: f64,                        // SHNU_RATE(매수비율)
    exec_volume_rate_vs_yesterday: f64,   // PRDY_VOL_VRSS_ACML_VOL_RATE(전일 거래량 대비 등락율)
    market_price_time: Time,              // OPRC_HOUR(시가 시간)
    vs_market_price_sign: VsPriceSign,    // OPRC_VRSS_PRPR_SIGN(시가대비구분)
    vs_market_price: u32,                 // OPRC_VRSS_PRPR(시가대비)
    upper_price_time: Time,               // HGPR_HOUR(최고가 시간)
    vs_upper_price_sign: VsPriceSign,     // HGPR_VRSS_PRPR_SIGN(고가대비구분)
    vs_upper_price: u32,                  // HGPR_VRSS_PRPR(고가대비)
    lower_price_time: Time,               // LWPR_HOUR(최저가 시간)
    vs_lower_price_sign: VsPriceSign,     // LWPR_VRSS_PRPR_SIGN(저가대비구분)
    vs_lower_price: u32,                  // LWPR_VRSS_PRPR(저가대비)
    business_operation_date: Time,        // BSOP_DATE(영업일자)
    new_market_operation_class_code: MarketOperationClassCode, // NEW_MKOP_CLS_CODE(신 장운영 구분 코드)
    trade_suspended: bool,                                     // TRHT_YN(거래정지 여부)
    ask_order_remained: u64,                                   // ASKP_RSQN1(매도호가 잔량1)
    bid_order_remained: u64,                                   // BIDP_RSQN1(매수호가 잔량1)
    total_ask_order_remained: u64,                             // TOTAL_ASKP_RSQN(총 매도호가 잔량)
    total_bid_order_remained: u64,                             // TOTAL_BIDP_RSQN(총 매수호가 잔량)
    turnover_ratio: f64,                                       // VOL_TNRT(거래량 회전율)
    yesterday_symmetric_time_accumulate_volume: u64, // PRDY_SMNS_HOUR_ACML_VOL(전일 동시간 누적 거래량)
    yesterday_symmetric_time_accumulate_volume_rate: f64, // PRDY_SMNS_HOUR_ACML_VOL_RATE(전일 동시간 누적 거래량 비율)
    time_class_code: TimeClassCode,                       // HOUR_CLS_CODE(시간 구분 코드)
    market_termination_class_code: MarketTerminationClassCode, // MRKT_TRTM_CLS_CODE(임의종료구분코드)
    vi_standard_price: u32,                                    // VI_STND_PRC(정적VI발동기준가)
}

impl Body {
    pub fn parse(obj: json::object::Object) -> Result<Self, Error> {
        Ok(Self {
            shortcode: get_json_inner(&obj, "MKSC_SHRN_ISCD")
                .as_str()
                .unwrap()
                .to_string(),
            exec_time: Time::parse(
                get_json_inner(&obj, "STCK_CNTG_HOUR").as_str().unwrap(),
                "%Y%m%d%H%M%S",
            )?,
            current_price: get_json_inner(&obj, "STCK_PRPR").as_u32().unwrap(),
            price_sign_vs_yesterday: get_json_inner(&obj, "PRDY_VRSS_SIGN")
                .as_str()
                .unwrap()
                .to_string()
                .into(),
            price_vs_yesterday: get_json_inner(&obj, "PRDY_VRSS").as_u32().unwrap(),
            price_rate_vs_yesterday: get_json_inner(&obj, "PRDY_CTRT").as_f32().unwrap(),
            weighted_average_price: get_json_inner(&obj, "WGHN_AVRG_STCK_PRC").as_u64().unwrap(),
            market_price: get_json_inner(&obj, "STCK_OPRC").as_u32().unwrap(),
            market_upper_price: get_json_inner(&obj, "STCK_HGPR").as_u32().unwrap(),
            market_lower_price: get_json_inner(&obj, "STCK_LWPR").as_u32().unwrap(),
            ask_price: get_json_inner(&obj, "ASKP1").as_u32().unwrap(),
            bid_price: get_json_inner(&obj, "BIDP1").as_u32().unwrap(),
            exec_volume: get_json_inner(&obj, "CNTG_VOL").as_u64().unwrap(),
            accumulative_exec_volume: get_json_inner(&obj, "ACML_VOL").as_u64().unwrap(),
            accumulative_exec_amount: get_json_inner(&obj, "ACML_TR_PBMN").as_u64().unwrap(),
            ask_exec_count: get_json_inner(&obj, "SELN_CNTG_CSNU").as_u32().unwrap(),
            bid_exec_count: get_json_inner(&obj, "SHNU_CNTG_CSNU").as_u32().unwrap(),
            natural_bid_exec_count: get_json_inner(&obj, "NTBY_CNTG_CSNU").as_u32().unwrap(),
            volume_power: get_json_inner(&obj, "CTTR").as_u64().unwrap(),
            total_ask_exec_volume: get_json_inner(&obj, "SELN_CNTG_SMTN").as_u64().unwrap(),
            total_bid_exec_volume: get_json_inner(&obj, "SHNU_CNTG_SMTN").as_u64().unwrap(),
            exec_class: get_json_inner(&obj, "CCLD_DVSN")
                .as_str()
                .unwrap()
                .to_string()
                .into(),
            bid_rate: get_json_inner(&obj, "SHNU_RATE").as_f64().unwrap(),
            exec_volume_rate_vs_yesterday: get_json_inner(&obj, "PRDY_VOL_VRSS_ACML_VOL_RATE")
                .as_f64()
                .unwrap(),
            market_price_time: Time::parse(
                get_json_inner(&obj, "OPRC_HOUR").as_str().unwrap(),
                "%Y%m%d%H%M%S",
            )?,
            vs_market_price_sign: get_json_inner(&obj, "OPRC_VRSS_PRPR_SIGN")
                .as_str()
                .unwrap()
                .to_string()
                .into(),
            vs_market_price: get_json_inner(&obj, "OPRC_VRSS_PRPR").as_u32().unwrap(),
            upper_price_time: Time::parse(
                get_json_inner(&obj, "HGPR_HOUR").as_str().unwrap(),
                "%Y%m%d%H%M%S",
            )?,
            vs_upper_price_sign: get_json_inner(&obj, "HGPR_VRSS_PRPR_SIGN")
                .as_str()
                .unwrap()
                .to_string()
                .into(),
            vs_upper_price: get_json_inner(&obj, "HGPR_VRSS_PRPR").as_u32().unwrap(),
            lower_price_time: Time::parse(
                get_json_inner(&obj, "LWPR_HOUR").as_str().unwrap(),
                "%Y%m%d%H%M%S",
            )?,
            vs_lower_price_sign: get_json_inner(&obj, "LWPR_VRSS_PRPR_SIGN")
                .as_str()
                .unwrap()
                .to_string()
                .into(),
            vs_lower_price: get_json_inner(&obj, "LWPR_VRSS_PRPR").as_u32().unwrap(),
            business_operation_date: Time::parse(
                get_json_inner(&obj, "BSOP_DATE").as_str().unwrap(),
                "%Y%m%d%H%M%S",
            )?,
            new_market_operation_class_code: get_json_inner(&obj, "NEW_MKOP_CLS_CODE")
                .as_str()
                .unwrap()
                .to_string()
                .into(),
            trade_suspended: get_json_inner(&obj, "TRHT_YN").as_bool().unwrap(),
            ask_order_remained: get_json_inner(&obj, "ASKP_RSQN1").as_u64().unwrap(),
            bid_order_remained: get_json_inner(&obj, "BIDP_RSQN1").as_u64().unwrap(),
            total_ask_order_remained: get_json_inner(&obj, "TOTAL_ASKP_RSQN").as_u64().unwrap(),
            total_bid_order_remained: get_json_inner(&obj, "TOTAL_BIDP_RSQN").as_u64().unwrap(),
            turnover_ratio: get_json_inner(&obj, "VOL_TNRT").as_f64().unwrap(),
            yesterday_symmetric_time_accumulate_volume: get_json_inner(
                &obj,
                "PRDY_SMNS_HOUR_ACML_VOL",
            )
            .as_u64()
            .unwrap(),
            yesterday_symmetric_time_accumulate_volume_rate: get_json_inner(
                &obj,
                "PRDY_SMNS_HOUR_ACML_VOL_RATE",
            )
            .as_f64()
            .unwrap(),
            time_class_code: get_json_inner(&obj, "HOUR_CLS_CODE")
                .as_str()
                .unwrap()
                .to_string()
                .into(),
            market_termination_class_code: get_json_inner(&obj, "MRKT_TRTM_CLS_CODE")
                .as_str()
                .unwrap()
                .to_string()
                .into(),
            vi_standard_price: get_json_inner(&obj, "VI_STND_PRC").as_u32().unwrap(),
        })
    }
}

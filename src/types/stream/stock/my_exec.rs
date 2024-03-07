use super::Header;
use crate::types::{parse_bool, Aes256CbcDec, CorrectionClass, Direction, OrderClass, Time};
use crate::util::get_json_inner;
use crate::{Error, BUF_SIZE};
use aes::cipher::{block_padding::ZeroPadding, BlockDecryptMut, KeyIvInit};
use base64::Engine;

#[derive(Debug, Clone)]
pub struct MyExec {
    header: Header,
    body: Option<Body>,
}

unsafe impl Send for MyExec {}

impl MyExec {
    pub fn parse(s: String, iv: String, key: String) -> Result<Self, Error> {
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
            let components = s.split('|').collect::<Vec<&str>>();
            let header_str = &components[0..3];
            let encrypted = header_str[0] == "1";
            let mut decrypted = [0u8; BUF_SIZE];
            let base64_decoded = match base64::prelude::BASE64_STANDARD
                .decode(components[3].to_string().into_bytes())
            {
                Ok(result) => result,
                Err(e) => {
                    error!("base64 decode failed: {:?}", e);
                    return Err(Error::AesInvalidLength);
                }
            };
            if encrypted {
                let decryptor = match Aes256CbcDec::new_from_slices(key.as_bytes(), iv.as_bytes()) {
                    Ok(decryptor) => decryptor,
                    Err(e) => {
                        error!("AES cipher error: {:?}", e);
                        return Err(Error::AesInvalidLength);
                    }
                };
                if let Err(e) =
                    decryptor.decrypt_padded_b2b_mut::<ZeroPadding>(&base64_decoded, &mut decrypted)
                {
                    error!("AES decrypt error: {:?}", e);
                    return Err(Error::AesDecryptError(e.to_string()));
                }
            }
            let decrypted = std::str::from_utf8(decrypted.as_slice())?;
            let splits = decrypted.split('^').collect::<Vec<&str>>();
            let business_operation_date = Time::now().date();
            let exec_time = Time::parse(&(business_operation_date + splits[11]), "%Y%m%d%H%M%S")?;
            let header = Header {
                tr_id: header_str[1].into(),
                datetime: exec_time.clone(),
            };
            let body = Some(Body {
                cust_id: splits[0].to_string(),
                acnt_no: splits[1].to_string(),
                order_no: splits[2].parse()?,
                origin_order_no: splits[3].parse().unwrap_or_else(|_| 0u64),
                ask_bid_class: splits[4].into(),
                correction_class: splits[5].into(),
                order_kind: splits[6].into(),
                order_condition: splits[7].to_string(),
                shortcode: splits[8].to_string(),
                exec_qty: splits[9].parse()?,
                exec_price: splits[10].parse()?,
                exec_time,
                is_refused: parse_bool(splits[12]),
                is_executed: parse_bool(splits[13]),
                is_accepted: parse_bool(splits[14]),
                branch_no: splits[15].to_string(),
                order_qty: splits[16].parse()?,
                account_name: splits[17].to_string(),
                stock_name: splits[18].to_string(),
                credit_class: splits[19].to_string(),
                credit_loan_date: match Time::parse(splits[20], "%Y%m%d") {
                    Ok(d) => Some(d),
                    Err(_) => None,
                },
                stock_name_40: splits[21].to_string(),
            });
            Ok(Self { header, body })
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn body(&self) -> &Option<Body> {
        &self.body
    }
}

#[derive(Debug, Clone)]
pub struct Body {
    pub cust_id: String,                   // CUST_ID(고객 ID)
    pub acnt_no: String,                   // ACNT_NO(계좌번호)
    pub order_no: u64,                     // ODER_NO(주문번호)
    pub origin_order_no: u64,              // OODER_NO(원주문번호)
    pub ask_bid_class: Direction,          // SELN_BYOV_CLS(매도매수구분)
    pub correction_class: CorrectionClass, // RCTF_CLS(정정구분)
    pub order_kind: OrderClass,            // ODER_KIND(주문종류)
    pub order_condition: String,           // ODER_COND(주문조건)
    pub shortcode: String,                 // STCK_SHRN_ISCD(주식 단축 종목코드)
    pub exec_qty: u64,                     // CNTG_QTY(체결 수량)
    pub exec_price: u32,                   // CNTG_UNPR(체결단가)
    pub exec_time: Time,                   // STCK_CNTG_HOUR(주식 체결 시간)
    pub is_refused: bool,                  // RFUS_YN(거부여부)
    pub is_executed: bool,                 // CNTG_YN(체결여부)
    pub is_accepted: bool,                 // ACPT_YN(접수여부)
    pub branch_no: String,                 // BRNC_NO(지점번호)
    pub order_qty: u64,                    // ODER_QTY(주문수량)
    pub account_name: String,              // ACNT_NAME(계좌명)
    pub stock_name: String,                // CNTG_ISNM(체결종목명)
    pub credit_class: String,              // CRDT_CLS(신용구분) TODO: CreditClass,
    pub credit_loan_date: Option<Time>,    // CRDT_LOAN_DATE(신용대출일자)
    pub stock_name_40: String,             // CNTG_ISNM40(체결종목명40)
}

use korea_investment_api::prelude::*;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;
use xan_log::init_logger;

#[macro_use]
extern crate log;

#[derive(StructOpt)]
#[structopt(name = "opt", about = "example")]
struct Opt {
    config_path: PathBuf,
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    TomlDeserializeError(#[from] toml::de::Error),
    #[error(transparent)]
    ApiError(#[from] korea_investment_api::Error),
}

fn get_config(path: &PathBuf) -> Result<Config, Error> {
    let mut buf = String::new();
    let mut fd = std::fs::File::open(path)?;
    let _len = fd.read_to_string(&mut buf)?;
    Ok(toml::from_str(&buf)?)
}

async fn get_api(config: &Config) -> Result<KoreaInvestmentApi, Error> {
    let account = Account {
        cano: config.cano().clone(),
        acnt_prdt_cd: config.acnt_prdt_cd().clone(),
    };
    Ok(KoreaInvestmentApi::new(
        config.environment().clone(),
        config.app_key(),
        config.app_secret(),
        account,
        config.hts_id(),
        config.token().clone(),
        config.approval_key().clone(),
        config.real_appkey().clone(),
        config.real_appsecret().clone(),
        config.real_token().clone(),
        config.real_approval_key().clone(),
    )
    .await?)
}

#[tokio::main]
async fn main() {
    let _ = init_logger();
    let Opt { config_path } = Opt::from_args();
    let mut config = get_config(&config_path).unwrap();
    let mut api = get_api(&config).await.unwrap();
    api.export_config(&config).unwrap();

    // {{{ 삼성전자 일자별 가격(단일 API 호출)
    let mut samsung_electronics_daily_prices = api
        .quote
        .daily_price(MarketCode::Stock, "005930", PeriodCode::ThirtyDays, false)
        .await
        .unwrap();
    if samsung_electronics_daily_prices.rt_cd() != "0" {
        if samsung_electronics_daily_prices.msg_cd() == "EGW00123" {
            warn!("만료된 토큰 이용: {:?}", samsung_electronics_daily_prices);

            // 토큰 재발급
            let token = api.auth.create_token().await.unwrap();

            // config 파일에 재발급 토큰 적용
            config.set_token(Some(token));
            api.export_config(&config).unwrap();

            // api 재호출
            samsung_electronics_daily_prices = api
                .quote
                .daily_price(MarketCode::Stock, "005930", PeriodCode::ThirtyDays, false)
                .await
                .unwrap();
        } else {
            panic!("Error Response: {:?}", samsung_electronics_daily_prices);
        }
    }
    info!(
        "삼성전자 일자별 가격 Response: {:?}",
        samsung_electronics_daily_prices
    );
    // }}}

    // {{{ 관심종목 그룹조회
    let groups = api
        .quote
        .group_list(quote::GroupListParameter::new(config.hts_id()))
        .await
        .unwrap();
    info!("관심종목 그룹조회 Response: {:?}", groups);

    if let Some(output) = groups.output() {
        for group in output {
            let group_items = api
                .quote
                .group_item(quote::GroupItemParameter::new(
                    config.hts_id(),
                    group.inter_grp_code(),
                ))
                .await
                .unwrap();
            debug!("관심종목 그룹별 종목조회 Response: {:?}", group_items);
        }
    } else if let Some(output) = groups.output2() {
        for group in output {
            let group_items = api
                .quote
                .group_item(quote::GroupItemParameter::new(
                    config.hts_id(),
                    group.inter_grp_code(),
                ))
                .await
                .unwrap();
            debug!("관심종목 그룹별 종목조회 Response: {:?}", group_items);
        }
    }
    // }}}

    // {{{ 삼성전자 기본조회 - 정규장만 가능
    let mut samsung_electronics_basic_info = api
        .quote
        .basic_stock_info(ProductTypeCode::Stock, "005930")
        .await
        .unwrap();
    if samsung_electronics_basic_info.rt_cd() != "0" {
        if samsung_electronics_basic_info.msg_cd() == "EGW00123" {
            warn!("만료된 토큰 이용: {:?}", samsung_electronics_basic_info);

            // 토큰 재발급
            let token = api.real_auth.create_token().await.unwrap();

            // config 파일에 재발급 토큰 적용
            config.set_real_token(Some(token));
            api.export_config(&config).unwrap();

            // api 재호출
            samsung_electronics_basic_info = api
                .quote
                .basic_stock_info(ProductTypeCode::Stock, "005930")
                .await
                .unwrap();
        } else {
            panic!("Error Response: {:?}", samsung_electronics_basic_info);
        }
    }
    info!(
        "주식 기본조회 Response: {:?}",
        samsung_electronics_basic_info
    );
    // }}}

    // {{{ 거래량 순위 - 정규장만 가능한데, 모의투자 Credential로도 호출이 됨.
    let volume_rank = api
        .quote
        .volume_rank(quote::VolumeRankParameter::new(
            String::from("005930"),
            ShareClassCode::Whole,
            BelongClassCode::IncreasedVolumeRate,
            TargetClassCode {
                margin_30: false,
                margin_40: false,
                margin_50: false,
                margin_60: false,
                margin_100: true,
                credit_30: false,
                credit_40: false,
                credit_50: false,
                credit_60: false,
            },
            TargetExeceptClassCode {
                overheat: true,
                administrated: true,
                settlement_trading: true,
                insufficient_posting: true,
                preferred_share: false,
                suspended: true,
            },
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    info!("거래량 순위 Response: {:?}", volume_rank);
    // }}}

    // {{{ 주문 테스트

    // 주문 테스트용 종목 하한가
    let the_enm_daily_prices = api
        .quote
        .daily_price(MarketCode::Stock, "089230", PeriodCode::ThirtyDays, false)
        .await
        .unwrap();
    let lower_price = Price::from(
        (the_enm_daily_prices
            .output()
            .clone()
            .unwrap()
            .first()
            .unwrap()
            .stck_clpr()
            .parse::<u64>()
            .unwrap() as f64
            * 0.8) as u32,
    )
    .ceil();
    let correction_price = lower_price.inc();

    // {{{ 주문 테스트 - KRX
    // [CAUTION] 실제로 하한가 주문 및 정정 주문이 발생합니다.

    // 신규 주문
    let order_result = api
        .order
        .order_cash(
            OrderClass::Limit,
            Direction::Bid,
            "089230", // The E&M
            Quantity::from(1),
            lower_price,
            Some(TargetExchange::KRX),
        )
        .await;
    info!("신규 주문 Response: {:?}", order_result);

    if let Ok(result) = order_result {
        if let Some(output) = result.output() {
            // 정정 주문
            let correct_result = api
                .order
                .correct(
                    OrderClass::Limit,
                    output.krx_fwdg_ord_orgno(),
                    output.odno(),
                    CorrectionClass::Correction,
                    true,
                    Quantity::from(1),
                    correction_price,
                    Some(TargetExchange::KRX),
                )
                .await;
            info!("정정 주문 Response: {:?}", correct_result);

            if let Ok(result) = correct_result {
                if let Some(output) = result.output() {
                    // 취소 주문
                    let cancel_result = api
                        .order
                        .correct(
                            OrderClass::Limit,
                            output.krx_fwdg_ord_orgno(),
                            output.odno(),
                            CorrectionClass::Cancel,
                            true,
                            Quantity::from(1),
                            correction_price,
                            Some(TargetExchange::KRX),
                        )
                        .await;
                    info!("취소 주문 Response: {:?}", cancel_result);
                }
            }
        }
    }
    // }}}

    // {{{ 주문 테스트 - NXT
    // [CAUTION] 실제로 하한가 주문 및 정정 주문이 발생합니다.
    if *config.environment() == Environment::Real {
        // 신규 주문
        let order_result = api
            .order
            .order_cash(
                OrderClass::Limit,
                Direction::Bid,
                "089230", // The E&M
                Quantity::from(1),
                lower_price,
                Some(TargetExchange::NXT),
            )
            .await;
        info!("신규 주문 Response: {:?}", order_result);

        if let Ok(result) = order_result {
            if let Some(output) = result.output() {
                // 정정 주문
                let correct_result = api
                    .order
                    .correct(
                        OrderClass::Limit,
                        output.krx_fwdg_ord_orgno(),
                        output.odno(),
                        CorrectionClass::Correction,
                        true,
                        Quantity::from(1),
                        correction_price,
                        Some(TargetExchange::NXT),
                    )
                    .await;
                info!("정정 주문 Response: {:?}", correct_result);

                if let Ok(result) = correct_result {
                    if let Some(output) = result.output() {
                        // 취소 주문
                        let cancel_result = api
                            .order
                            .correct(
                                OrderClass::Limit,
                                output.krx_fwdg_ord_orgno(),
                                output.odno(),
                                CorrectionClass::Cancel,
                                true,
                                Quantity::from(1),
                                correction_price,
                                Some(TargetExchange::NXT),
                            )
                            .await;
                        info!("취소 주문 Response: {:?}", cancel_result);
                    }
                }
            }
        }
    }
    // }}}

    // {{{ 주문 테스트 - SOR
    // [CAUTION] 실제로 하한가 주문 및 정정 주문이 발생합니다.
    if *config.environment() == Environment::Real {
        // 신규 주문
        let order_result = api
            .order
            .order_cash(
                OrderClass::Limit,
                Direction::Bid,
                "089230", // The E&M
                Quantity::from(1),
                lower_price,
                Some(TargetExchange::SOR),
            )
            .await;
        info!("신규 주문 Response: {:?}", order_result);

        if let Ok(result) = order_result {
            if let Some(output) = result.output() {
                // 정정 주문
                let correct_result = api
                    .order
                    .correct(
                        OrderClass::Limit,
                        output.krx_fwdg_ord_orgno(),
                        output.odno(),
                        CorrectionClass::Correction,
                        true,
                        Quantity::from(1),
                        correction_price,
                        Some(TargetExchange::SOR),
                    )
                    .await;
                info!("정정 주문 Response: {:?}", correct_result);

                if let Ok(result) = correct_result {
                    if let Some(output) = result.output() {
                        // 취소 주문
                        let cancel_result = api
                            .order
                            .correct(
                                OrderClass::Limit,
                                output.krx_fwdg_ord_orgno(),
                                output.odno(),
                                CorrectionClass::Cancel,
                                true,
                                Quantity::from(1),
                                correction_price,
                                Some(TargetExchange::SOR),
                            )
                            .await;
                        info!("취소 주문 Response: {:?}", cancel_result);
                    }
                }
            }
        }
    }
    // }}}

    // {{{ 주문 테스트(신용) - KRX
    // [CAUTION] 실제로 하한가 주문 및 정정 주문이 발생합니다.

    // 신규 주문
    let order_result = api
        .order
        .order_credit(
            OrderClass::Limit,
            Direction::Bid,
            "089230", // The E&M
            Quantity::from(1),
            lower_price,
            Some(TargetExchange::KRX),
            CreditType::ProprietaryMarginLoanOpen,
        )
        .await;
    info!("신규 신용 주문 Response: {:?}", order_result);

    if let Ok(result) = order_result {
        if let Some(output) = result.output() {
            // 정정 주문
            let correct_result = api
                .order
                .correct(
                    OrderClass::Limit,
                    output.krx_fwdg_ord_orgno(),
                    output.odno(),
                    CorrectionClass::Correction,
                    true,
                    Quantity::from(1),
                    correction_price,
                    Some(TargetExchange::KRX),
                )
                .await;
            info!("정정 주문 Response: {:?}", correct_result);

            if let Ok(result) = correct_result {
                if let Some(output) = result.output() {
                    // 취소 주문
                    let cancel_result = api
                        .order
                        .correct(
                            OrderClass::Limit,
                            output.krx_fwdg_ord_orgno(),
                            output.odno(),
                            CorrectionClass::Cancel,
                            true,
                            Quantity::from(1),
                            correction_price,
                            Some(TargetExchange::KRX),
                        )
                        .await;
                    info!("취소 주문 Response: {:?}", cancel_result);
                }
            }
        }
    }
    // }}}

    // }}}

    // {{{ 주식잔고조회
    let balance = api
        .order
        .inquire_balance(None, None, None, None, None)
        .await
        .unwrap();
    if balance.rt_cd() != "0" {
        panic!("잔고조회 Error Response: {:?}", balance);
    }
    info!("주식잔고조회 Response: {:?}", balance);

    // 보유종목 출력
    for item in balance.output1() {
        info!(
            "보유종목: {} ({}) 수량: {} 평가손익: {}",
            item.prdt_name(),
            item.pdno(),
            item.hldg_qty(),
            item.evlu_pfls_amt(),
        );
    }

    // 계좌 요약 출력
    for summary in balance.output2() {
        info!(
            "총평가금액: {}, 순자산: {}, 예수금: {}",
            summary.tot_evlu_amt(),
            summary.nass_amt(),
            summary.dnca_tot_amt(),
        );
    }
    // }}}

    // {{{ 삼성전자 매수가능조회
    // 정확한 매수가능수량을 보려면 ORD_DVSN:01(시장가) + 단가 공란으로 조회
    // (지정가는 종목증거금율이 반영되지 않음)
    let psbl_order = api
        .order
        .inquire_psbl_order(
            "005930",           // 삼성전자
            None,               // 주문단가: 시장가 조회 시 공란
            OrderClass::Market, // 주문구분: 시장가
            false,              // CMA평가금액 포함여부
            false,              // 해외 포함여부
        )
        .await
        .unwrap();
    if psbl_order.rt_cd() != "0" {
        panic!("매수가능조회 Error Response: {:?}", psbl_order);
    }
    info!("매수가능조회 Response: {:?}", psbl_order);

    if let Some(output) = psbl_order.output() {
        info!(
            "주문가능현금: {}, 미수없는매수금액: {} (수량: {}), 최대매수금액: {} (수량: {})",
            output.ord_psbl_cash(),
            output.nrcvb_buy_amt(),
            output.nrcvb_buy_qty(),
            output.max_buy_amt(),
            output.max_buy_qty(),
        );
    }
    // }}}

    // {{{ 삼성전자 호가 실시간 시세 구독(호가; KRX)
    let (rx, subscribe_response) = api
        .k_data
        .subscribe_market::<Ordb, OrdbBody>("005930", TrId::RealtimeOrdbKrx)
        .await
        .unwrap();
    info!("호가 실시간 시세 구독 Response: {:?}", subscribe_response);

    // 구독한 시세 읽기
    let mut i = 0;
    if let Some(mut rx) = rx {
        while let Ok(ordb) = rx.recv().await {
            trace!("[실시간] KRX 호가 수신: {:?}", ordb);
            i += 1;
            if i == 10 {
                break;
            }
        }
    }
    // 시세 구독 해체
    let unsubscribe_response = api
        .k_data
        .unsubscribe_market("005930", TrId::RealtimeOrdbKrx)
        .await;
    info!("시세 구독 해제 Result: {:?}", unsubscribe_response);
    // }}}

    // {{{ 삼성전자 호가 실시간 시세 구독(체결; KRX)
    let (rx, subscribe_response) = api
        .k_data
        .subscribe_market::<Exec, ExecBody>("005930", TrId::RealtimeExecKrx)
        .await
        .unwrap();
    info!("체결 실시간 시세 구독 Response: {:?}", subscribe_response);

    // 구독한 시세 읽기
    let mut i = 0;
    if let Some(mut rx) = rx {
        while let Ok(exec) = rx.recv().await {
            trace!("[실시간] KRX 체결 수신: {:?}", exec);
            i += 1;
            if i == 10 {
                break;
            }
        }
    }
    // 시세 구독 해체
    let unsubscribe_response = api
        .k_data
        .unsubscribe_market("005930", TrId::RealtimeExecNxt)
        .await;
    info!("시세 구독 해제 Result: {:?}", unsubscribe_response);
    // }}}

    // {{{ 삼성전자 호가 실시간 시세 구독(호가; NXT)
    let (rx, subscribe_response) = api
        .k_data
        .subscribe_market::<Ordb, OrdbBody>("005930", TrId::RealtimeOrdbNxt)
        .await
        .unwrap();
    info!("호가 실시간 시세 구독 Response: {:?}", subscribe_response);

    // 구독한 시세 읽기
    let mut i = 0;
    if let Some(mut rx) = rx {
        while let Ok(ordb) = rx.recv().await {
            trace!("[실시간] NXT 호가 수신: {:?}", ordb);
            i += 1;
            if i == 10 {
                break;
            }
        }
    }
    // 시세 구독 해체
    let unsubscribe_response = api
        .k_data
        .unsubscribe_market("005930", TrId::RealtimeOrdbNxt)
        .await;
    info!("시세 구독 해제 Result: {:?}", unsubscribe_response);
    // }}}

    // {{{ 삼성전자 호가 실시간 시세 구독(체결; NXT)
    let (rx, subscribe_response) = api
        .k_data
        .subscribe_market::<Exec, ExecBody>("005930", TrId::RealtimeExecNxt)
        .await
        .unwrap();
    info!("체결 실시간 시세 구독 Response: {:?}", subscribe_response);

    // 구독한 시세 읽기
    let mut i = 0;
    if let Some(mut rx) = rx {
        while let Ok(exec) = rx.recv().await {
            trace!("[실시간] NXT 체결 수신: {:?}", exec);
            i += 1;
            if i == 10 {
                break;
            }
        }
    }
    // 시세 구독 해체
    let unsubscribe_response = api
        .k_data
        .unsubscribe_market("005930", TrId::RealtimeExecNxt)
        .await;
    info!("시세 구독 해제 Result: {:?}", unsubscribe_response);
    // }}}

    // {{{ 삼성전자 호가 실시간 시세 구독(호가; Both)
    let (rx, subscribe_response) = api
        .k_data
        .subscribe_market::<Ordb, OrdbBody>("005930", TrId::RealtimeOrdbUnion)
        .await
        .unwrap();
    info!("호가 실시간 시세 구독 Response: {:?}", subscribe_response);

    // 구독한 시세 읽기
    let mut i = 0;
    if let Some(mut rx) = rx {
        while let Ok(ordb) = rx.recv().await {
            trace!("[실시간] 통합 호가 수신: {:?}", ordb);
            i += 1;
            if i == 10 {
                break;
            }
        }
    }
    // 시세 구독 해체
    let unsubscribe_response = api
        .k_data
        .unsubscribe_market("005930", TrId::RealtimeOrdbUnion)
        .await;
    info!("시세 구독 해제 Result: {:?}", unsubscribe_response);
    // }}}

    // {{{ 삼성전자 호가 실시간 시세 구독(체결; Both)
    let (rx, subscribe_response) = api
        .k_data
        .subscribe_market::<Exec, ExecBody>("005930", TrId::RealtimeExecUnion)
        .await
        .unwrap();
    info!("체결 실시간 시세 구독 Response: {:?}", subscribe_response);
    // 구독한 시세 읽기
    let mut i = 0;
    if let Some(mut rx) = rx {
        while let Ok(exec) = rx.recv().await {
            trace!("[실시간] 통합 체결 수신: {:?}", exec);
            i += 1;
            if i == 10 {
                break;
            }
        }
    }
    // 시세 구독 해체
    let unsubscribe_response = api
        .k_data
        .unsubscribe_market("005930", TrId::RealtimeExecUnion)
        .await;
    info!("시세 구독 해제 Result: {:?}", unsubscribe_response);
    // }}}
}

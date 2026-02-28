use korea_investment_api::types::config::Config;
use korea_investment_api::types::request::stock::quote::{
    GroupItemParameter, GroupListParameter, VolumeRankParameter,
};
use korea_investment_api::types::{
    Account, BelongClassCode, MarketCode, PeriodCode, ProductTypeCode, ShareClassCode,
    TargetClassCode, TargetExeceptClassCode,
};
use korea_investment_api::KoreaInvestmentApi;
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

    // 삼성전자 일자별 가격(단일 API 호출)
    let mut samsung_electronics_daily_prices = api
        .quote
        .daily_price(MarketCode::Stock, "000040", PeriodCode::ThirtyDays, false)
        .await
        .unwrap();
    if samsung_electronics_daily_prices.rt_cd() != "0" {
        if samsung_electronics_daily_prices.msg_cd() == "EGW00123" {
            warn!("만료된 토큰 이용: {:?}", samsung_electronics_daily_prices);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            // 토큰 재발급
            let token = api.auth.create_token().await.unwrap();

            // config 파일에 재발급 토큰 적용
            config.set_token(Some(token));
            api.export_config(&config).unwrap();

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            // api 재호출
            samsung_electronics_daily_prices = api
                .quote
                .daily_price(MarketCode::Stock, "000040", PeriodCode::ThirtyDays, false)
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

    let groups = api
        .quote
        .group_list(GroupListParameter::new(config.hts_id()))
        .await
        .unwrap();
    info!("관심종목 그룹조회 Response: {:?}", groups);

    if let Some(output) = groups.output() {
        for group in output {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let group_items = api
                .quote
                .group_item(GroupItemParameter::new(
                    config.hts_id(),
                    group.inter_grp_code(),
                ))
                .await
                .unwrap();
            debug!("관심종목 그룹별 종목조회 Response: {:?}", group_items);
        }
    } else if let Some(output) = groups.output2() {
        for group in output {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let group_items = api
                .quote
                .group_item(GroupItemParameter::new(
                    config.hts_id(),
                    group.inter_grp_code(),
                ))
                .await
                .unwrap();
            debug!("관심종목 그룹별 종목조회 Response: {:?}", group_items);
        }
    }

    // 삼성전자 기본조회 - 정규장만 가능
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let samsung_electronics_basic_info = api
        .quote
        .basic_stock_info(ProductTypeCode::Stock, "000040")
        .await
        .unwrap();
    info!(
        "주식 기본조회 Response: {:?}",
        samsung_electronics_basic_info
    );

    // 거래량 순위 - 정규장만 가능한데, 모의투자 Credential로도 호출이 됨.
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let volume_rank = api
        .quote
        .volume_rank(VolumeRankParameter::new(
            String::from("000040"),
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

    //    // 주문 테스트
    //    // [CAUTION] 실제로 하한가 주문 및 정정 주문이 발생합니다.
    //
    //    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    //
    //    // 하한가
    //    let lower_price = Price::from(
    //        (samsung_electronics_daily_prices
    //            .output()
    //            .clone()
    //            .unwrap()
    //            .first()
    //            .unwrap()
    //            .stck_clpr()
    //            .parse()
    //            .unwrap() as f64
    //            * 0.8) as u32,
    //    )
    //    .ceil();
    //
    //    // 신규 주문
    //    let order_result = api
    //        .order
    //        .order_cash(
    //            OrderClass::Limit,
    //            Direction::Bid,
    //            "000040",
    //            Quantity::from(1),
    //            lower_price,
    //        )
    //        .await;
    //    info!("신규 주문 Response: {:?}", order_result);
    //
    //    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    //
    //    if let Ok(result) = order_result {
    //        if let Some(output) = result.output() {
    //            // 정정 주문
    //            let new_price = lower_price.inc();
    //            let correct_result = api
    //                .order
    //                .correct(
    //                    OrderClass::Limit,
    //                    output.krx_fwdg_ord_orgno(),
    //                    output.odno(),
    //                    CorrectionClass::Correction,
    //                    true,
    //                    Quantity::from(1),
    //                    new_price,
    //                )
    //                .await;
    //            info!("정정 주문 Response: {:?}", correct_result);
    //
    //            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    //
    //            if let Ok(result) = correct_result {
    //                if let Some(output) = result.output() {
    //                    // 취소 주문
    //                    let cancel_result = api
    //                        .order
    //                        .correct(
    //                            OrderClass::Limit,
    //                            output.krx_fwdg_ord_orgno(),
    //                            output.odno(),
    //                            CorrectionClass::Cancel,
    //                            true,
    //                            Quantity::from(1),
    //                            new_price,
    //                        )
    //                        .await;
    //                    info!("취소 주문 Response: {:?}", cancel_result);
    //                }
    //            }
    //        }
    //    }
    //
    //    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    //
    //    // 삼성전자 호가 실시간 시세 구독
    //    let (rx, subscribe_response) = api
    //        .k_data
    //        .subscribe_market::<Ordb, OrdbBody>("005930", TrId::RealtimeOrdb)
    //        .await
    //        .unwrap();
    //    info!("호가 실시간 시세 구독 Response: {:?}", subscribe_response);
    //
    //    // 구독한 시세 읽기
    //    let mut i = 0;
    //    if let Some(mut rx) = rx {
    //        while let Some(ordb) = rx.recv().await {
    //            debug!("[실시간] 호가 수신: {:?}", ordb);
    //            i += 1;
    //            if i == 10 {
    //                break;
    //            }
    //        }
    //    }
    //    // 시세 구독 해체
    //    let unsubscribe_response = api
    //        .k_data
    //        .unsubscribe_market("005930", TrId::RealtimeOrdb)
    //        .await;
    //    info!("시세 구독 해제 Result: {:?}", unsubscribe_response);
}

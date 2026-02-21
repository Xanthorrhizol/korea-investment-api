use korea_investment_api::types::config::Config;
use korea_investment_api::types::request::stock::quote::{GroupItemParameter, GroupListParameter};
use korea_investment_api::types::stream::stock::{ordb::Body as OrdbBody, Ordb};
use korea_investment_api::types::{
    Account, CorrectionClass, Direction, MarketCode, OrderClass, PeriodCode, Price, Quantity, TrId,
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
    )
    .await?)
}

#[tokio::main]
async fn main() {
    let _ = init_logger();
    let Opt { config_path } = Opt::from_args();
    let config = get_config(&config_path).unwrap();
    let mut api = get_api(&config).await.unwrap();
    api.export_config(&config).unwrap();

    // 삼성전자 일자별 가격(단일 API 호출)
    let samsung_electronics_daily_prices = api
        .quote
        .daily_price(MarketCode::Stock, "005930", PeriodCode::ThirtyDays, false)
        .await
        .unwrap();
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
    let last_close_price: u64 = samsung_electronics_daily_prices
        .output()
        .clone()
        .unwrap()
        .first()
        .unwrap()
        .stck_clpr()
        .parse()
        .unwrap();

    // 주문 테스트
    // [CAUTION] 실제로 하한가 주문 및 정정 주문이 발생합니다.
    let order_result = api
        .order
        .order_cash(
            OrderClass::Limit,
            Direction::Bid,
            "005930",
            Quantity::from(1),
            Price::from((last_close_price as f64 * 0.8) as u32).ceil(),
        )
        .await;
    info!("신규 주문 Response: {:?}", order_result);
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    if let Ok(result) = order_result {
        if let Some(output) = result.output() {
            let correct_result = api
                .order
                .correct(
                    OrderClass::Limit,
                    output.krx_fwdg_ord_orgno(),
                    output.odno(),
                    CorrectionClass::Correction,
                    true,
                    Quantity::from(1),
                    Price::from((last_close_price as f64 * 0.8) as u32)
                        .ceil()
                        .inc(),
                )
                .await;
            info!("정정 주문 Response: {:?}", correct_result);
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            let cancel_result = api
                .order
                .correct(
                    OrderClass::Limit,
                    output.krx_fwdg_ord_orgno(),
                    output.odno(),
                    CorrectionClass::Cancel,
                    true,
                    Quantity::from(1),
                    Price::from(60000),
                )
                .await;
            info!("취소 주문 Response: {:?}", cancel_result);
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    }

    // 삼성전자 호가 실시간 시세 구독
    let (rx, _subscribe_response) = api
        .k_data
        .subscribe_market::<Ordb, OrdbBody>("KR7005930003", TrId::RealtimeOrdb)
        .unwrap();

    // 구독한 시세 읽기
    let mut i = 0;
    if let Some(mut rx) = rx {
        while let Some(ordb) = rx.recv().await {
            debug!("[실시간] 호가 수신: {:?}", ordb);
            if i == 10 {
                break;
            }
            i += 1;
        }
    }
}

use korea_investment_api::types::config::Config;
use korea_investment_api::types::request::stock::quote::{GroupItemParameter, GroupListParameter};
use korea_investment_api::types::stream::stock::{ordb::Body as OrdbBody, Ordb};
use korea_investment_api::types::{Account, MarketCode, PeriodCode, TrId};
use korea_investment_api::KoreaInvestmentApi;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;
use xan_log::init_logger;

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
    init_logger();
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
    println!("{:?}", samsung_electronics_daily_prices);

    let groups = api
        .quote
        .group_list(GroupListParameter::new(config.hts_id()))
        .await
        .unwrap();
    println!("{:?}", groups);

    if let Some(output) = groups.output() {
        for group in output {
            let group_items = api
                .quote
                .group_item(GroupItemParameter::new(
                    config.hts_id(),
                    group.inter_grp_code(),
                ))
                .await
                .unwrap();
            println!("{:?}", group_items);
        }
    } else if let Some(output) = groups.output2() {
        for group in output {
            let group_items = api
                .quote
                .group_item(GroupItemParameter::new(
                    config.hts_id(),
                    group.inter_grp_code(),
                ))
                .await
                .unwrap();
            println!("{:?}", group_items);
        }
    }

    // 삼성전자 호가 실시간 시세 구독
    let (rx, subscribe_response) = api
        .k_data
        .subscribe_market::<Ordb, OrdbBody>("KR7005930003", TrId::RealtimeOrdb)
        .unwrap();

    // 구독한 시세 읽기
    if let Some(mut rx) = rx {
        while let Some(ordb) = rx.recv().await {
            println!("Got orderbook: {:?}", ordb);
        }
    }
}

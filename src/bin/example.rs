use korea_investment_api::types::config::Config;
use korea_investment_api::types::{Account, MarketCode, PeriodCode, TrId};
use korea_investment_api::KoreaInvestmentApi;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

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
    )
    .await?)
}

#[tokio::main]
async fn main() {
    let Opt { config_path } = Opt::from_args();
    let config = get_config(&config_path).unwrap();
    let mut api = get_api(&config).await.unwrap();

    // 삼성전자 일자별 가격(단일 API 호출)
    let samsung_electronics_daily_prices = api
        .quote
        .daily_price(MarketCode::Stock, "005930", PeriodCode::ThirtyDays, false)
        .await
        .unwrap();

    // 삼성전자 호가 실시간 시세 구독
    let subscribe_response = api
        .k_data
        .subscribe_market("KR7005930003", TrId::RealtimeOrdb)
        .unwrap();

    // 구독한 시세 읽기
    while let Ok(ordb) = api.k_data.ordb_recv() {
        println!("Got orderbook: {:?}", ordb);
    }
}

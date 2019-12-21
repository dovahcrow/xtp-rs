use dotenv::dotenv;
use env_logger::init;
use failure::Fallible;
use log::{error, info, warn};
use std::net::SocketAddrV4;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;
use xtp::{
    OrderBookStruct, QuoteApi, QuoteSpi, XTPExchangeType, XTPLogLevel, XTPMarketDataStruct,
    XTPProtocolType, XTPRspInfoStruct, XTPSpecificTickerStruct, XTPTickByTickStruct,
};

type XTPST = XTPSpecificTickerStruct;
type XTPRI = XTPRspInfoStruct;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of xtp-rs usage.")]
struct Args {
    #[structopt(short, long, default_value = "1")]
    id: i8,
    #[structopt(short, long, env = "XTP_SERVER_ADDR")]
    server_addr: SocketAddrV4,
    #[structopt(short, long, env = "XTP_USERNAME")]
    username: String,
    #[structopt(short, long, env = "XTP_PASSWORD")]
    password: String,
    #[structopt(long, default_value = "/tmp")]
    path: String,
}

fn main() -> Fallible<()> {
    dotenv()?;
    init();

    let args = Args::from_args();

    let mut api = QuoteApi::new(123, &args.path, XTPLogLevel::Trace);

    info!("XTP Version: {:?}", api.get_api_version());
    info!("Trading Day: {:?}", api.get_trading_day());

    api.register_spi(MySpi);

    api.set_heart_beat_interval(10);
    api.set_udp_buffer_size(1024);

    api.login(
        args.server_addr,
        &args.username,
        &args.password,
        XTPProtocolType::TCP,
    )?;

    api.subscribe_market_data(&["600120", "600090"], XTPExchangeType::SH)?;
    api.subscribe_order_book(&["600018"], XTPExchangeType::SH)?;
    api.subscribe_tick_by_tick(&["600018", "600021"], XTPExchangeType::SH)?;

    sleep(Duration::from_secs(100));

    api.logout()?;

    Ok(())
}

struct MySpi;

impl QuoteSpi for MySpi {
    fn on_error(&self, error_info: XTPRI) {
        error!("{:?}", error_info);
    }

    fn on_disconnected(&self, reason: i32) {
        warn!("Disconnected, reason: {}", reason)
    }

    fn on_sub_market_data(&self, ticker: XTPST, error_info: XTPRI, is_last: bool) {
        info!(
            "Sub Market Data: {:?}: {:?} {}",
            ticker, error_info, is_last
        );
    }

    fn on_sub_order_book(&self, ticker: XTPST, error_info: XTPRI, is_last: bool) {
        info!("Sub Orderbook: {:?}: {:?} {}", ticker, error_info, is_last);
    }

    fn on_sub_tick_by_tick(&self, ticker: XTPST, error_info: XTPRI, is_last: bool) {
        info!(
            "Sub Tick By Tick: {:?}: {:?} {}",
            ticker, error_info, is_last
        );
    }

    fn on_depth_market_data(
        &self,
        market_data: XTPMarketDataStruct,
        bid1_qty: &[i64],
        max_bid1_count: i32,
        ask1_qty: &[i64],
        max_ask1_count: i32,
    ) {
        info!(
            "Market Depth: {:?}, {:?}, {}, {:?}, {}",
            market_data, bid1_qty, max_bid1_count, ask1_qty, max_ask1_count
        );
    }
    fn on_tick_by_tick(&self, tbt_data: XTPTickByTickStruct) {
        info!("Tick by tick: {:?}", tbt_data);
    }

    fn on_order_book(&self, ob: OrderBookStruct) {
        info!("Orderbook: {:?}", ob);
    }
}

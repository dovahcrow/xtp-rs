use dotenv::dotenv;
use failure::Fallible;
use std::net::SocketAddrV4;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;
use xtp::{
    OrderBookStruct, QuoteApi, QuoteSpi, XTPExchangeType, XTPLogLevel, XTPProtocolType,
    XTPRspInfoStruct, XTPSpecificTickerStruct, XTPTickByTickStruct,
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
    let args = Args::from_args();

    let mut api = QuoteApi::new(123, &args.path, XTPLogLevel::Trace);

    println!("XTP Version: {:?}", api.get_api_version());
    println!("Trading Day: {:?}", api.get_trading_day());

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
    fn on_sub_market_data(&self, ticker: XTPST, error_info: XTPRI, is_last: bool) {
        println!(
            "Sub Market Data: {:?}: {:?} {}",
            ticker, error_info, is_last
        );
    }

    fn on_sub_order_book(&self, ticker: XTPST, error_info: XTPRI, is_last: bool) {
        println!("Sub Orderbook: {:?}: {:?} {}", ticker, error_info, is_last);
    }

    fn on_sub_tick_by_tick(&self, ticker: XTPST, error_info: XTPRI, is_last: bool) {
        println!(
            "Sub Tick By Tick: {:?}: {:?} {}",
            ticker, error_info, is_last
        );
    }

    fn on_tick_by_tick(&self, tbt_data: XTPTickByTickStruct) {
        println!("Tick by tick: {:?}", tbt_data);
    }

    fn on_order_book(&self, ob: OrderBookStruct) {
        println!("Orderbook: {:?}", ob);
    }
}

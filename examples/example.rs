use dotenv::dotenv;
use failure::Fallible;
use std::net::SocketAddrV4;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;
use xtp::sys::XTPOB;
use xtp::{QuoteApi, QuoteSpi, XTPExchangeType, XTPLogLevel, XTPProtocolType, XTPRspInfoStruct};

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

    let mut api = QuoteApi::new(123, &args.path, XTPLogLevel::Info);

    println!("{:?}", api.get_api_version());
    println!("{:?}", api.get_trading_day());

    api.register_spi(MySpi);

    api.login(
        args.server_addr,
        &args.username,
        &args.password,
        XTPProtocolType::TCP,
    )?;

    api.subscribe_all_order_book(XTPExchangeType::SH)?;

    sleep(Duration::from_secs(10));

    api.logout()?;

    Ok(())
}

struct MySpi;

impl QuoteSpi for MySpi {
    fn on_order_book(&self, order_book: &XTPOB) {
        println!("Orderbook info");
    }

    fn on_subscribe_all_order_book(
        &self,
        exchange_id: XTPExchangeType,
        error_info: XTPRspInfoStruct,
    ) {
        println!("{:?}: {:?}", exchange_id, error_info);
    }
}

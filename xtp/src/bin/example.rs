use xtp::{QuoteApi, QuoteSpi, XTPLogLevel};

fn main() {
    let mut api = QuoteApi::new(123, "/tmp", XTPLogLevel::Info);

    println!("{:?}", api.get_api_version());
    println!("{:?}", api.get_trading_day());
    println!("{:?}", api.get_api_last_error());

    api.register_spi(MySpi);
}

struct MySpi;

impl QuoteSpi for MySpi {}

mod errors;
mod quote_api;
mod quote_spi;
#[doc(hidden)]
pub mod sys;
mod types;

pub use errors::XTPError;
pub use quote_api::QuoteApi;
pub use quote_spi::QuoteSpi;
#[doc(hidden)]
#[no_mangle]
pub use quote_spi::{
    QuoteSpiStub_Rust_Destructor, QuoteSpiStub_Rust_OnDisconnected, QuoteSpiStub_Rust_OnError,
    QuoteSpiStub_Rust_OnSubMarketData, QuoteSpiStub_Rust_OnSubscribeAllOptionTickByTick,
    QuoteSpiStub_Rust_OnUnSubscribeAllMarketData,
    QuoteSpiStub_Rust_OnUnSubscribeAllOptionOrderBook,
    QuoteSpiStub_Rust_OnUnSubscribeAllOptionTickByTick,
    QuoteSpiStub_Rust_OnUnSubscribeAllOrderBook,
};
pub use types::{
    XTPExchangeType, XTPLogLevel, XTPMarketType, XTPOrderActionStatusType, XTPPositionEffectType,
    XTPPriceType, XTPProtocolType, XTPRspInfoStruct, XTPSideType,
};

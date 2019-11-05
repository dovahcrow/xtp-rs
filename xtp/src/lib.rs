mod quote_api;
mod quote_spi;

pub use quote_api::QuoteApi;
pub use quote_spi::QuoteSpi;
#[doc(hidden)]
pub use quote_spi::{
    QuoteSpiStub_Rust_OnDisconnected, QuoteSpiStub_Rust_OnError, QuoteSpiStub_Rust_OnSubMarketData,
};

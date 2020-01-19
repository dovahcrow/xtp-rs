//! # xtp
//!
//! `xtp` is a Rust binding for the [XTP SDK](http://xtp.zts.com.cn),
//! which is an interface for trading on the Chinese stock market.
//!
//! ## Current status
//!
//! All APIs are implemented but are partially tested/documented. USE AT YOUR OWN RISK!
//!
//! ## Dependencies
//!
//! * This crate depends on `libxtpquoteapi` and `libxtptraderapi` from the [**bundled** XTP SDK](http://xtp.zts.com.cn). Please install them first.
//! (The **Installation** here means adding the path to the correponding libs to `LD_LIBRARY` on linux, or `%PATH%` on windows.)
//! The version of the installed lib should be same as the one in the [bundled sdk](http://github.com/dovahcrow/xtp-sdk) in case of incompatibility.
//!
//! * Only 64 bit of the SDK is implemented.
//!
//! * For windows users, please install 64bit rust compiler and llvm (required by [rust-bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html)).
//!
//! ## Usage
//!
//! Please check out the [examples](https://github.com/dovahcrow/xtp-rs/tree/master/examples) folder for detail usage.
//!

mod errors;
mod quote_api;
mod quote_spi;
mod sys;
mod trader_api;
mod trader_spi;
mod types;

pub use errors::XTPError;
pub use quote_api::QuoteApi;
pub use quote_spi::QuoteSpi;
pub use sys::{TXTPOrderTypeType, TXTPTradeTypeType};
pub use trader_api::TraderApi;
pub use trader_spi::TraderSpi;
pub use types::{
    ETFReplaceType, OrderBookStruct, XTPAccountType, XTPBusinessType, XTPExchangeType,
    XTPFundOperStatus, XTPFundTransferNotice, XTPFundTransferReq, XTPFundTransferType, XTPLogLevel,
    XTPMarketDataStruct, XTPMarketType, XTPMarketdataType, XTPOptCallOrPutType,
    XTPOptExerciseTypeType, XTPOrderActionStatusType, XTPOrderCancelInfo, XTPOrderInfo,
    XTPOrderInsertInfo, XTPOrderStatusType, XTPOrderSubmitStatusType, XTPPositionDirectionType,
    XTPPositionEffectType, XTPPriceType, XTPProtocolType, XTPQueryAssetRsp, XTPQueryETFBaseReq,
    XTPQueryETFBaseRsp, XTPQueryETFComponentReq, XTPQueryETFComponentRsp,
    XTPQueryFundTransferLogReq, XTPQueryIPOQuotaRsp, XTPQueryIPOTickerRsp,
    XTPQueryOptionAuctionInfoReq, XTPQueryOptionAuctionInfoRsp, XTPQueryOrderByPageReq,
    XTPQueryOrderReq, XTPQueryStkPositionRsp, XTPQueryStructuredFundInfoReq,
    XTPQueryTraderByPageReq, XTPQueryTraderReq, XTPQuoteStaticInfo, XTPRspInfoStruct, XTPSideType,
    XTPSpecificTickerStruct, XTPSplitMergeStatus, XTPStructuredFundInfo, XTPTbtType,
    XTPTeResumeType, XTPTickByTickStruct, XTPTickerPriceInfo, XTPTickerType, XTPTradeReport,
};

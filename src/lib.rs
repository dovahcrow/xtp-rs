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
pub use quote_spi::{
    QuoteSpiStub_Rust_Destructor, QuoteSpiStub_Rust_OnDisconnected, QuoteSpiStub_Rust_OnError,
    QuoteSpiStub_Rust_OnSubMarketData, QuoteSpiStub_Rust_OnSubscribeAllOptionTickByTick,
    QuoteSpiStub_Rust_OnUnSubscribeAllMarketData,
    QuoteSpiStub_Rust_OnUnSubscribeAllOptionOrderBook,
    QuoteSpiStub_Rust_OnUnSubscribeAllOptionTickByTick,
    QuoteSpiStub_Rust_OnUnSubscribeAllOrderBook,
};
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

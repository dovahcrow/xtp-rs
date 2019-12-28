use crate::sys::{
    XTPFundTransferNotice, XTPOrderCancelInfo, XTPOrderInfo, XTPQueryAssetRsp, XTPQueryETFBaseRsp,
    XTPQueryETFComponentRsp, XTPQueryIPOQuotaRsp, XTPQueryIPOTickerRsp,
    XTPQueryOptionAuctionInfoRsp, XTPQueryOrderRsp, XTPQueryStkPositionRsp, XTPQueryTradeRsp,
    XTPStructuredFundInfo, XTPTradeReport, XTPRI,
};
use crate::types;
use crate::types::FromRaw;
use libc::{c_int, c_void};
use std::slice::from_raw_parts;

type RXTPRI = types::XTPRspInfoStruct;
type RXTPET = types::XTPExchangeType;
type RXTPST = types::XTPSpecificTickerStruct;
type RXTPOB = types::OrderBookStruct;
type RXTPTBT = types::XTPTickByTickStruct;
type RXTPQSI = types::XTPQuoteStaticInfo;
type RXTPTPI = types::XTPTickerPriceInfo;
type RXTPMD = types::XTPMarketDataStruct;

#[allow(unused_variables)]
pub trait TraderSpi {
    fn on_disconnected(&self, session_id: u64, reason: i32) {}
    fn on_error(&self, error_info: RXTPRI) {}
    fn on_order_event(&self, order_info: types::XTPOrderInfo, error_info: RXTPRI, session_id: u64) {
    }
}

unsafe fn unwrap_quote_spi<'a>(spi: *mut c_void) -> &'a mut dyn TraderSpi {
    &mut **(spi as *mut *mut dyn TraderSpi)
}

// ***************** Rust Implementations for C++ Use **********************

#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnDisconnected(
    spi: *mut c_void,
    session_id: u64,
    reason: c_int,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    spi.on_disconnected(session_id, reason as i32);
}

#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnError(spi: *mut c_void, error_info: *const XTPRI) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_error(error_info);
}

#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnOrderEvent(
    spi: *mut c_void,
    order_info: *const XTPOrderInfo,
    error_info: *const XTPRI,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let order_info = unsafe { types::XTPOrderInfo::from_raw(&*order_info) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_order_event(order_info, error_info, session_id)
}

#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnTradeEvent(
    spi: *mut c_void,
    Xtrade_info: *const XTPTradeReport,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}

#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnCancelOrderError(
    spi: *mut c_void,
    cancel_info: *const XTPOrderCancelInfo,
    error_info: *const XTPRI,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryOrder(
    spi: *mut c_void,
    order_info: *const XTPQueryOrderRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryOrderByPage(
    spi: *mut c_void,
    order_info: *const XTPQueryOrderRsp,
    req_count: i64,
    order_sequence: i64,
    query_reference: i64,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryTrade(
    spi: *mut c_void,
    trade_info: *const XTPQueryTradeRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryTradeByPage(
    spi: *mut c_void,
    trade_info: *const XTPQueryTradeRsp,
    req_count: i64,
    trade_sequence: i64,
    query_reference: i64,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryPosition(
    spi: *mut c_void,
    position: *const XTPQueryStkPositionRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryAsset(
    spi: *mut c_void,
    asset: *const XTPQueryAssetRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryStructuredFund(
    spi: *mut c_void,
    fund_info: *const XTPStructuredFundInfo,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryFundTransfer(
    spi: *mut c_void,
    fund_transfer_info: *const XTPFundTransferNotice,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnFundTransfer(
    spi: *mut c_void,
    fund_transfer_info: *const XTPFundTransferNotice,
    error_info: *const XTPRI,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryETF(
    spi: *mut c_void,
    etf_info: *const XTPQueryETFBaseRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryETFBasket(
    spi: *mut c_void,
    etf_component_info: *const XTPQueryETFComponentRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryIPOInfoList(
    spi: *mut c_void,
    ipo_info: *const XTPQueryIPOTickerRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryIPOQuotaInfo(
    spi: *mut c_void,
    quota_info: *const XTPQueryIPOQuotaRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_OnQueryOptionAuctionInfo(
    spi: *mut c_void,
    option_info: *const XTPQueryOptionAuctionInfoRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
}
#[no_mangle]
pub extern "C" fn TraderSpiStub_Rust_Destructor(spi: *mut c_void) {
    let spi = spi as *mut Box<dyn TraderSpi>;

    let _: Box<Box<dyn TraderSpi>> = unsafe { Box::from_raw(spi) };
}

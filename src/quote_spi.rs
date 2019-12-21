use crate::sys::{XTPMD, XTPOB, XTPQSI, XTPRI, XTPST, XTPTBT, XTPTPI, XTP_EXCHANGE_TYPE};
use crate::types;
use crate::types::FromRaw;
use libc::{c_int, c_void};
use std::convert::TryFrom;
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
pub trait QuoteSpi {
    fn on_disconnected(&self, reason: i32) {}
    fn on_error(&self, error_info: RXTPRI) {}
    fn on_sub_market_data(&self, ticker: RXTPST, error_info: RXTPRI, is_last: bool) {}
    fn on_unsub_market_data(&self, ticker: RXTPST, error_info: RXTPRI, is_last: bool) {}
    fn on_depth_market_data(
        &self,
        market_data: RXTPMD,
        bid1_qty: &[i64],
        max_bid1_count: i32,
        ask1_qty: &[i64],
        max_ask1_count: i32,
    ) {
    }
    fn on_sub_order_book(&self, ticker: RXTPST, error_info: RXTPRI, is_last: bool) {}
    fn on_unsub_order_book(&self, ticker: RXTPST, error_info: RXTPRI, is_last: bool) {}
    fn on_order_book(&self, _order_book: RXTPOB) {}
    fn on_sub_tick_by_tick(&self, ticker: RXTPST, error_info: RXTPRI, is_last: bool) {}
    fn on_unsub_tick_by_tick(&self, ticker: RXTPST, error_info: RXTPRI, is_last: bool) {}
    fn on_tick_by_tick(&self, tbt_data: RXTPTBT) {}
    fn on_subscribe_allmarket_data(&self, exchange_id: RXTPET, error_info: RXTPRI) {}
    fn on_unsubscribe_allmarket_data(&self, exchange_id: RXTPET, error_info: RXTPRI) {}
    fn on_subscribe_all_order_book(&self, exchange_id: RXTPET, error_info: RXTPRI) {}
    fn on_unsubscribe_all_order_book(&self, exchange_id: RXTPET, error_info: RXTPRI) {}
    fn on_subscribe_all_tick_by_tick(&self, exchange_id: RXTPET, error_info: RXTPRI) {}
    fn on_unsubscribe_all_tick_by_tick(&self, exchange_id: RXTPET, error_info: RXTPRI) {}
    fn on_query_alltickers(&self, ticker_info: RXTPQSI, error_info: RXTPRI, is_last: bool) {}
    fn on_querytickers_price_info(&self, ticker_info: RXTPTPI, error_info: RXTPRI, is_last: bool) {}
    fn on_subscribe_all_optionmarket_data(&self, exchange_id: RXTPET, error_info: RXTPRI) {}
    fn on_unsubscribe_all_optionmarket_data(&self, exchange_id: RXTPET, error_info: RXTPRI) {}
    fn on_subscribe_all_option_order_book(&self, exchange_id: RXTPET, error_info: RXTPRI) {}
    fn on_unsubscribe_all_option_order_book(&self, exchange_id: RXTPET, error_info: RXTPRI) {}
    fn on_subscribe_all_option_tick_by_tick(&self, exchange_id: RXTPET, error_info: RXTPRI) {}
    fn on_unsubscribe_all_option_tick_by_tick(&self, exchange_id: RXTPET, error_info: RXTPRI) {}
}

unsafe fn unwrap_quote_spi<'a>(spi: *mut c_void) -> &'a mut dyn QuoteSpi {
    &mut **(spi as *mut *mut dyn QuoteSpi)
}

// ***************** Rust Implementations for C++ Use **********************
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnDisconnected(spi: *mut c_void, reason: c_int) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    spi.on_disconnected(reason as i32);
}

#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnError(spi: *mut c_void, error_info: *const XTPRI) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_error(error_info);
}

#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnSubMarketData(
    spi: *mut c_void,
    ticker: *const XTPST,
    error_info: *const XTPRI,
    is_last: bool,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let ticker = unsafe { RXTPST::from_raw(&*ticker) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_sub_market_data(ticker, error_info, is_last);
}

#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnUnSubMarketData(
    spi: *mut c_void,
    ticker: *const XTPST,
    error_info: *const XTPRI,
    is_last: bool,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let ticker = unsafe { RXTPST::from_raw(&*ticker) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_unsub_market_data(ticker, error_info, is_last);
}

#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnDepthMarketData(
    spi: *mut c_void,
    market_data: *const XTPMD,
    bid1_qty: *const i64,
    bid1_count: i32,
    max_bid1_count: i32,
    ask1_qty: *const i64,
    ask1_count: i32,
    max_ask1_count: i32,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };

    let market_data = unsafe { RXTPMD::from_raw(&*market_data) };
    let bid1_qty = unsafe { from_raw_parts(bid1_qty, bid1_count as usize) };
    let ask1_qty = unsafe { from_raw_parts(ask1_qty, ask1_count as usize) };

    spi.on_depth_market_data(
        market_data,
        bid1_qty,
        max_bid1_count,
        ask1_qty,
        max_ask1_count,
    );
}

#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnSubOrderBook(
    spi: *mut c_void,
    ticker: *const XTPST,
    error_info: *const XTPRI,
    is_last: bool,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    let ticker = unsafe { RXTPST::from_raw(&*ticker) };
    spi.on_sub_order_book(ticker, error_info, is_last);
}

#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnUnSubOrderBook(
    spi: *mut c_void,
    ticker: *const XTPST,
    error_info: *const XTPRI,
    is_last: bool,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let ticker = unsafe { RXTPST::from_raw(&*ticker) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_unsub_order_book(ticker, error_info, is_last);
}

#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnOrderBook(spi: *mut c_void, order_book: *const XTPOB) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let order_book = unsafe { RXTPOB::from_raw(&*order_book) };
    spi.on_order_book(order_book);
}

#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnSubTickByTick(
    spi: *mut c_void,
    ticker: *const XTPST,
    error_info: *const XTPRI,
    is_last: bool,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let ticker = unsafe { RXTPST::from_raw(&*ticker) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_sub_tick_by_tick(ticker, error_info, is_last);
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnUnSubTickByTick(
    spi: *mut c_void,
    ticker: *const XTPST,
    error_info: *const XTPRI,
    is_last: bool,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let ticker = unsafe { RXTPST::from_raw(&*ticker) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_unsub_tick_by_tick(ticker, error_info, is_last);
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnTickByTick(spi: *mut c_void, tbt_data: *const XTPTBT) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let tbt_data = unsafe { RXTPTBT::from_raw(&*tbt_data) };
    spi.on_tick_by_tick(tbt_data);
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnSubscribeAllMarketData(
    spi: *mut c_void,
    exchange_id: XTP_EXCHANGE_TYPE,
    error_info: *const XTPRI,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let exchange_id = unsafe { RXTPET::from_raw(exchange_id) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_subscribe_allmarket_data(exchange_id, error_info);
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnUnSubscribeAllMarketData(
    spi: *mut c_void,
    exchange_id: XTP_EXCHANGE_TYPE,
    error_info: *const XTPRI,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let exchange_id = unsafe { RXTPET::from_raw(exchange_id) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_unsubscribe_allmarket_data(exchange_id, error_info);
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnSubscribeAllOrderBook(
    spi: *mut c_void,
    exchange_id: XTP_EXCHANGE_TYPE,
    error_info: *const XTPRI,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let exchange_id = unsafe { RXTPET::from_raw(exchange_id) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_subscribe_all_order_book(exchange_id, error_info);
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnUnSubscribeAllOrderBook(
    spi: *mut c_void,
    exchange_id: XTP_EXCHANGE_TYPE,
    error_info: *const XTPRI,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let exchange_id = unsafe { RXTPET::from_raw(exchange_id) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_unsubscribe_all_order_book(exchange_id, error_info);
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnSubscribeAllTickByTick(
    spi: *mut c_void,
    exchange_id: XTP_EXCHANGE_TYPE,
    error_info: *const XTPRI,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let exchange_id = unsafe { RXTPET::from_raw(exchange_id) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_subscribe_all_tick_by_tick(exchange_id, error_info)
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnUnSubscribeAllTickByTick(
    spi: *mut c_void,
    exchange_id: XTP_EXCHANGE_TYPE,
    error_info: *const XTPRI,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let exchange_id = unsafe { RXTPET::from_raw(exchange_id) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_unsubscribe_all_tick_by_tick(exchange_id, error_info);
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnQueryAllTickers(
    spi: *mut c_void,
    ticker_info: *const XTPQSI,
    error_info: *const XTPRI,
    is_last: bool,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let ticker_info = unsafe { RXTPQSI::from_raw(&*ticker_info) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_query_alltickers(ticker_info, error_info, is_last);
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnQueryTickersPriceInfo(
    spi: *mut c_void,
    ticker_info: *const XTPTPI,
    error_info: *const XTPRI,
    is_last: bool,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let ticker_info = unsafe { RXTPTPI::from_raw(&*ticker_info) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_querytickers_price_info(ticker_info, error_info, is_last);
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnSubscribeAllOptionMarketData(
    spi: *mut c_void,
    exchange_id: XTP_EXCHANGE_TYPE,
    error_info: *const XTPRI,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let exchange_id = unsafe { RXTPET::from_raw(exchange_id) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_subscribe_all_optionmarket_data(exchange_id, error_info)
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnUnSubscribeAllOptionMarketData(
    spi: *mut c_void,
    exchange_id: XTP_EXCHANGE_TYPE,
    error_info: *const XTPRI,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let exchange_id = unsafe { RXTPET::from_raw(exchange_id) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_unsubscribe_all_optionmarket_data(exchange_id, error_info)
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnSubscribeAllOptionOrderBook(
    spi: *mut c_void,
    exchange_id: XTP_EXCHANGE_TYPE,
    error_info: *const XTPRI,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let exchange_id = unsafe { RXTPET::from_raw(exchange_id) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_subscribe_all_option_order_book(exchange_id, error_info)
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnUnSubscribeAllOptionOrderBook(
    spi: *mut c_void,
    exchange_id: XTP_EXCHANGE_TYPE,
    error_info: *const XTPRI,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let exchange_id = unsafe { RXTPET::from_raw(exchange_id) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_unsubscribe_all_option_order_book(exchange_id, error_info)
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnSubscribeAllOptionTickByTick(
    spi: *mut c_void,
    exchange_id: XTP_EXCHANGE_TYPE,
    error_info: *const XTPRI,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let exchange_id = unsafe { RXTPET::from_raw(exchange_id) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_subscribe_all_option_tick_by_tick(exchange_id, error_info);
}
#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnUnSubscribeAllOptionTickByTick(
    spi: *mut c_void,
    exchange_id: XTP_EXCHANGE_TYPE,
    error_info: *const XTPRI,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let exchange_id = unsafe { RXTPET::from_raw(exchange_id) };
    let error_info = unsafe { RXTPRI::from_raw(&*error_info) };
    spi.on_unsubscribe_all_option_tick_by_tick(exchange_id, error_info);
}

#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_Destructor(spi: *mut c_void) {
    let spi = spi as *mut Box<dyn QuoteSpi>;

    let _: Box<Box<dyn QuoteSpi>> = unsafe { Box::from_raw(spi) };
}

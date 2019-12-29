use crate::sys::{
    XTPFundTransferNotice, XTPOrderCancelInfo, XTPOrderInfo, XTPQueryAssetRsp, XTPQueryETFBaseRsp,
    XTPQueryETFComponentRsp, XTPQueryIPOQuotaRsp, XTPQueryIPOTickerRsp,
    XTPQueryOptionAuctionInfoRsp, XTPQueryOrderRsp, XTPQueryStkPositionRsp, XTPQueryTradeRsp,
    XTPStructuredFundInfo, XTPTradeReport, XTPRI,
};
use crate::types;
use crate::types::FromRaw;
use libc::{c_int, c_void};

type RXTPRI<'a> = types::XTPRspInfoStruct<'a>;

#[allow(unused_variables)]
pub trait TraderSpi {
    fn on_disconnected(&self, session_id: u64, reason: i32) {}
    fn on_error(&self, error_info: RXTPRI) {}
    fn on_order_event(&self, order_info: types::XTPOrderInfo, error_info: RXTPRI, session_id: u64) {
    }
    fn on_trade_event(&self, x_trade_info: types::XTPTradeReport, session_id: u64) {}
    fn on_cancel_order_error(
        &self,
        cancel_info: types::XTPOrderCancelInfo,
        error_info: RXTPRI,
        session_id: u64,
    ) {
    }
    fn on_query_order(
        &self,
        order_info: types::XTPOrderInfo,
        error_info: RXTPRI,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
    fn on_query_order_by_page(
        &self,
        order_info: types::XTPOrderInfo,
        req_count: i64,
        order_sequence: i64,
        query_reference: i64,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
    fn on_query_trade(
        &self,
        trade_info: types::XTPTradeReport,
        error_info: RXTPRI,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
    fn on_query_trade_by_page(
        &self,
        trade_info: types::XTPTradeReport,
        req_count: i64,
        trade_sequence: i64,
        query_reference: i64,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
    fn on_query_position(
        &self,
        position: types::XTPQueryStkPositionRsp,
        error_info: RXTPRI,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
    fn on_query_asset(
        &self,
        asset: types::XTPQueryAssetRsp,
        error_info: RXTPRI,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
    fn on_query_structured_fund(
        &self,
        fund_info: types::XTPStructuredFundInfo,
        error_info: RXTPRI,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
    fn on_query_fund_transfer(
        &self,
        fund_transfer_info: types::XTPFundTransferNotice,
        error_info: RXTPRI,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
    fn on_fund_transfer(
        &self,
        fund_transfer_info: types::XTPFundTransferNotice,
        error_info: RXTPRI,
        session_id: u64,
    ) {
    }
    fn on_query_etf(
        &self,
        etf_info: types::XTPQueryETFBaseRsp,
        error_info: RXTPRI,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
    fn on_query_etf_basket(
        &self,
        etf_component_info: types::XTPQueryETFComponentRsp,
        error_info: RXTPRI,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
    fn on_query_ipo_info_list(
        &self,
        ipo_info: types::XTPQueryIPOTickerRsp,
        error_info: RXTPRI,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
    fn on_query_ipo_quota_info(
        &self,
        quota_info: types::XTPQueryIPOQuotaRsp,
        error_info: RXTPRI,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
    fn on_query_option_auction_info(
        &self,
        option_info: types::XTPQueryOptionAuctionInfoRsp,
        error_info: RXTPRI,
        request_id: i32,
        is_last: bool,
        session_id: u64,
    ) {
    }
}

unsafe fn unwrap_trader_spi<'a>(spi: *mut c_void) -> &'a mut dyn TraderSpi {
    &mut **(spi as *mut *mut dyn TraderSpi)
}

// ***************** Rust Implementations for C++ Use **********************

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnDisconnected(
    spi: *mut c_void,
    session_id: u64,
    reason: c_int,
) {
    let spi = unwrap_trader_spi(spi);
    spi.on_disconnected(session_id, reason as i32);
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnError(spi: *mut c_void, error_info: *const XTPRI) {
    let spi = unwrap_trader_spi(spi);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_error(error_info);
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnOrderEvent(
    spi: *mut c_void,
    order_info: *const XTPOrderInfo,
    error_info: *const XTPRI,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let order_info = types::XTPOrderInfo::from_raw(&*order_info);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_order_event(order_info, error_info, session_id)
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnTradeEvent(
    spi: *mut c_void,
    x_trade_info: *const XTPTradeReport,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let x_trade_info = types::XTPTradeReport::from_raw(&*x_trade_info);
    spi.on_trade_event(x_trade_info, session_id)
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnCancelOrderError(
    spi: *mut c_void,
    cancel_info: *const XTPOrderCancelInfo,
    error_info: *const XTPRI,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let cancel_info = types::XTPOrderCancelInfo::from_raw(&*cancel_info);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_cancel_order_error(cancel_info, error_info, session_id)
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryOrder(
    spi: *mut c_void,
    order_info: *const XTPQueryOrderRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let order_info = types::XTPOrderInfo::from_raw(&*order_info);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_query_order(order_info, error_info, request_id, is_last, session_id)
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryOrderByPage(
    spi: *mut c_void,
    order_info: *const XTPQueryOrderRsp,
    req_count: i64,
    order_sequence: i64,
    query_reference: i64,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let order_info = types::XTPOrderInfo::from_raw(&*order_info);
    spi.on_query_order_by_page(
        order_info,
        req_count,
        order_sequence,
        query_reference,
        request_id,
        is_last,
        session_id,
    )
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryTrade(
    spi: *mut c_void,
    trade_info: *const XTPQueryTradeRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let trade_info = types::XTPTradeReport::from_raw(&*trade_info);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_query_trade(trade_info, error_info, request_id, is_last, session_id)
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryTradeByPage(
    spi: *mut c_void,
    trade_info: *const XTPQueryTradeRsp,
    req_count: i64,
    trade_sequence: i64,
    query_reference: i64,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let trade_info = types::XTPTradeReport::from_raw(&*trade_info);
    spi.on_query_trade_by_page(
        trade_info,
        req_count,
        trade_sequence,
        query_reference,
        request_id,
        is_last,
        session_id,
    )
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryPosition(
    spi: *mut c_void,
    position: *const XTPQueryStkPositionRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let position = types::XTPQueryStkPositionRsp::from_raw(&*position);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_query_position(position, error_info, request_id, is_last, session_id)
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryAsset(
    spi: *mut c_void,
    asset: *const XTPQueryAssetRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let asset = types::XTPQueryAssetRsp::from_raw(&*asset);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_query_asset(asset, error_info, request_id, is_last, session_id)
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryStructuredFund(
    spi: *mut c_void,
    fund_info: *const XTPStructuredFundInfo,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let fund_info = types::XTPStructuredFundInfo::from_raw(&*fund_info);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_query_structured_fund(fund_info, error_info, request_id, is_last, session_id)
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryFundTransfer(
    spi: *mut c_void,
    fund_transfer_info: *const XTPFundTransferNotice,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let fund_transfer_info = types::XTPFundTransferNotice::from_raw(&*fund_transfer_info);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_query_fund_transfer(
        fund_transfer_info,
        error_info,
        request_id,
        is_last,
        session_id,
    )
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnFundTransfer(
    spi: *mut c_void,
    fund_transfer_info: *const XTPFundTransferNotice,
    error_info: *const XTPRI,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let fund_transfer_info = types::XTPFundTransferNotice::from_raw(&*fund_transfer_info);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_fund_transfer(fund_transfer_info, error_info, session_id)
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryETF(
    spi: *mut c_void,
    etf_info: *const XTPQueryETFBaseRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let etf_info = types::XTPQueryETFBaseRsp::from_raw(&*etf_info);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_query_etf(etf_info, error_info, request_id, is_last, session_id)
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryETFBasket(
    spi: *mut c_void,
    etf_component_info: *const XTPQueryETFComponentRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let etf_component_info = types::XTPQueryETFComponentRsp::from_raw(&*etf_component_info);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_query_etf_basket(
        etf_component_info,
        error_info,
        request_id,
        is_last,
        session_id,
    )
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryIPOInfoList(
    spi: *mut c_void,
    ipo_info: *const XTPQueryIPOTickerRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let ipo_info = types::XTPQueryIPOTickerRsp::from_raw(&*ipo_info);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_query_ipo_info_list(ipo_info, error_info, request_id, is_last, session_id)
}
#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryIPOQuotaInfo(
    spi: *mut c_void,
    quota_info: *const XTPQueryIPOQuotaRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let quota_info = types::XTPQueryIPOQuotaRsp::from_raw(&*quota_info);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_query_ipo_quota_info(quota_info, error_info, request_id, is_last, session_id)
}
#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_OnQueryOptionAuctionInfo(
    spi: *mut c_void,
    option_info: *const XTPQueryOptionAuctionInfoRsp,
    error_info: *const XTPRI,
    request_id: c_int,
    is_last: bool,
    session_id: u64,
) {
    let spi = unwrap_trader_spi(spi);
    let option_info = types::XTPQueryOptionAuctionInfoRsp::from_raw(&*option_info);
    let error_info = RXTPRI::from_raw(&*error_info);
    spi.on_query_option_auction_info(option_info, error_info, request_id, is_last, session_id)
}

#[no_mangle]
pub unsafe extern "C" fn TraderSpiStub_Rust_Destructor(spi: *mut c_void) {
    let spi = spi as *mut Box<dyn TraderSpi>;
    let _: Box<Box<dyn TraderSpi>> = Box::from_raw(spi);
}

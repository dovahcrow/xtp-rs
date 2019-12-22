use crate::errors::XTPError;
use crate::sys::{
    CreateTraderApi, TraderApi_CancelOrder, TraderApi_FundTransfer, TraderApi_GetAccountByXTPID,
    TraderApi_GetApiLastError, TraderApi_GetApiVersion, TraderApi_GetClientIDByXTPID,
    TraderApi_GetTradingDay, TraderApi_InsertOrder, TraderApi_IsServerRestart, TraderApi_Login,
    TraderApi_Logout, TraderApi_QueryAsset, TraderApi_QueryETF, TraderApi_QueryETFTickerBasket,
    TraderApi_QueryFundTransfer, TraderApi_QueryIPOInfoList, TraderApi_QueryIPOQuotaInfo,
    TraderApi_QueryOptionAuctionInfo, TraderApi_QueryOrderByXTPID, TraderApi_QueryOrders,
    TraderApi_QueryOrdersByPage, TraderApi_QueryPosition, TraderApi_QueryStructuredFund,
    TraderApi_QueryTrades, TraderApi_QueryTradesByPage, TraderApi_QueryTradesByXTPID,
    TraderApi_RegisterSpi, TraderApi_Release, TraderApi_SetHeartBeatInterval,
    TraderApi_SetSoftwareKey, TraderApi_SetSoftwareVersion, TraderApi_SubscribePublicTopic,
    TraderSpiStub, XTP_API_TraderApi, XTP_API_TraderSpi, XTP_LOG_LEVEL,
};
use crate::trader_spi::TraderSpi;
use crate::types;
use crate::types::FromRaw;
use failure::Fallible;
use libc::c_void;
use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::net::SocketAddrV4;

pub struct TraderApi {
    trader_api: *mut XTP_API_TraderApi,
    trader_spi_stub: Option<*mut TraderSpiStub>, // Free the stub after we freed XTP_API_QuoteApi in drop()
}

impl TraderApi {
    fn release(&mut self) {
        unsafe { TraderApi_Release(self.trader_api) };
    }

    pub fn get_trading_day(&mut self) -> &str {
        let ptr = unsafe { TraderApi_GetTradingDay(self.trader_api) }; // The string is freed by them
        unsafe { CStr::from_ptr(ptr) }.to_str().unwrap()
    }

    pub fn register_spi<T: TraderSpi>(&mut self, spi: T) {
        let trait_object_box: Box<Box<dyn TraderSpi>> = Box::new(Box::new(spi));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<dyn TraderSpi> as *mut c_void;

        let quote_spi_stub = unsafe { TraderSpiStub::new(trait_object_pointer) };

        let ptr = Box::into_raw(Box::new(quote_spi_stub));
        self.trader_spi_stub = Some(ptr);
        unsafe { TraderApi_RegisterSpi(self.trader_api, ptr as *mut XTP_API_TraderSpi) };
    }

    pub fn get_api_last_error(&mut self) -> types::XTPRspInfoStruct {
        unsafe { types::XTPRspInfoStruct::from_raw(&*TraderApi_GetApiLastError(self.trader_api)) }
    }

    pub fn get_api_version(&mut self) -> &CStr {
        let ptr = unsafe { TraderApi_GetApiVersion(self.trader_api) }; // The string is freed by them
        unsafe { CStr::from_ptr(ptr) }
    }

    pub fn get_client_id_by_xtpid(&mut self, order_xtp_id: u64) -> u8 {
        unsafe { TraderApi_GetClientIDByXTPID(self.trader_api, order_xtp_id) }
    }

    pub fn get_account_by_xtpid(&mut self, order_xtp_id: u64) -> &CStr {
        let ptr = unsafe { TraderApi_GetAccountByXTPID(self.trader_api, order_xtp_id) };
        unsafe { CStr::from_ptr(ptr) }
    }

    pub fn subscribe_public_topic(&mut self, resume_type: types::XTPTeResumeType) {
        unsafe { TraderApi_SubscribePublicTopic(self.trader_api, resume_type.into()) };
    }

    pub fn set_software_version(&mut self, version: &str) -> Fallible<()> {
        let version = CString::new(version)?;
        unsafe { TraderApi_SetSoftwareVersion(self.trader_api, version.as_ptr()) };
        Ok(())
    }

    pub fn set_software_key(&mut self, key: &str) -> Fallible<()> {
        let key = CString::new(key)?;
        unsafe { TraderApi_SetSoftwareKey(self.trader_api, key.as_ptr()) };
        Ok(())
    }

    pub fn set_heart_beat_interval(&mut self, interval: u32) {
        unsafe { TraderApi_SetHeartBeatInterval(self.trader_api, interval) }
    }

    pub fn login(
        &mut self,
        server_addr: SocketAddrV4,
        username: &str,
        password: &str,
        sock_type: types::XTPProtocolType,
    ) -> Fallible<i64> {
        {
            let ip = CString::new(format!("{}", server_addr.ip()))?;
            let username = CString::new(username)?;
            let password = CString::new(password)?;
            let ret_code = unsafe {
                TraderApi_Login(
                    self.trader_api,
                    ip.as_ptr(),
                    server_addr.port() as i32,
                    username.as_ptr(),
                    password.as_ptr(),
                    sock_type.into(),
                )
            };
            self.translate_code(ret_code as i64, false)
        }
    }

    pub fn logout(&mut self, session_id: u64) -> Fallible<i64> {
        let retc = unsafe { TraderApi_Logout(self.trader_api, session_id) };
        self.translate_code(retc as i64, true)
    }

    pub fn is_server_restart(&mut self, session_id: u64) -> bool {
        unsafe { TraderApi_IsServerRestart(self.trader_api, session_id) }
    }

    pub fn insert_order(
        &mut self,
        order: &types::XTPOrderInsertInfo,
        session_id: u64,
    ) -> Fallible<i64> {
        let mut order = order.into();
        let retc =
            unsafe { TraderApi_InsertOrder(self.trader_api, &mut order as *mut _, session_id) };
        self.translate_code(retc as i64, false)
    }

    pub fn cancel_order(&mut self, order_xtp_id: u64, session_id: u64) -> Fallible<i64> {
        let retc = unsafe { TraderApi_CancelOrder(self.trader_api, order_xtp_id, session_id) };
        self.translate_code(retc as i64, false)
    }

    pub fn query_order_by_xtpid(
        &mut self,
        order_xtp_id: u64,
        session_id: u64,
        request_id: i32,
    ) -> Fallible<i64> {
        let retc = unsafe {
            TraderApi_QueryOrderByXTPID(self.trader_api, order_xtp_id, session_id, request_id)
        };
        self.translate_code(retc as i64, true)
    }

    pub fn query_orders(
        &mut self,
        query_param: &types::XTPQueryOrderReq,
        session_id: u64,
        request_id: i32,
    ) -> Fallible<i64> {
        let mut query_param = query_param.into();

        let retc = unsafe {
            TraderApi_QueryOrders(
                self.trader_api,
                &mut query_param as *mut _,
                session_id,
                request_id,
            )
        };
        self.translate_code(retc as i64, true)
    }

    pub fn query_orders_by_page(
        &mut self,
        query_param: &types::XTPQueryOrderByPageReq,
        session_id: u64,
        request_id: i32,
    ) -> Fallible<i64> {
        let mut query_param = query_param.into();

        let retc = unsafe {
            TraderApi_QueryOrdersByPage(
                self.trader_api,
                &mut query_param as *mut _,
                session_id,
                request_id,
            )
        };
        self.translate_code(retc as i64, true)
    }

    pub fn query_trades_by_xtpid(
        &mut self,
        order_xtp_id: u64,
        session_id: u64,
        request_id: i32,
    ) -> Fallible<i64> {
        let retc = unsafe {
            TraderApi_QueryTradesByXTPID(self.trader_api, order_xtp_id, session_id, request_id)
        };
        self.translate_code(retc as i64, true)
    }

    pub fn query_trades(
        &mut self,
        query_param: &types::XTPQueryTraderReq,
        session_id: u64,
        request_id: i32,
    ) -> Fallible<i64> {
        let retc = unsafe {
            TraderApi_QueryTrades(
                self.trader_api,
                &mut query_param.into() as *mut _,
                session_id,
                request_id,
            )
        };
        self.translate_code(retc as i64, true)
    }

    pub fn query_trades_by_page(
        &mut self,
        query_param: &types::XTPQueryTraderByPageReq,
        session_id: u64,
        request_id: i32,
    ) -> Fallible<i64> {
        let retc = unsafe {
            TraderApi_QueryTradesByPage(
                self.trader_api,
                &mut query_param.into() as *mut _,
                session_id,
                request_id,
            )
        };

        self.translate_code(retc as i64, true)
    }

    pub fn query_position(
        &mut self,
        ticker: &str,
        session_id: u64,
        request_id: i32,
    ) -> Fallible<i64> {
        let s = CString::new(ticker).unwrap();

        let retc =
            unsafe { TraderApi_QueryPosition(self.trader_api, s.as_ptr(), session_id, request_id) };
        self.translate_code(retc as i64, true)
    }

    pub fn query_asset(&mut self, session_id: u64, request_id: i32) -> Fallible<i64> {
        let retc = unsafe { TraderApi_QueryAsset(self.trader_api, session_id, request_id) };
        self.translate_code(retc as i64, true)
    }

    pub fn query_structured_fund(
        &mut self,
        query_param: &types::XTPQueryStructuredFundInfoReq,
        session_id: u64,
        request_id: i32,
    ) -> Fallible<i64> {
        let retc = unsafe {
            TraderApi_QueryStructuredFund(
                self.trader_api,
                &mut query_param.into() as *mut _,
                session_id,
                request_id,
            )
        };
        self.translate_code(retc as i64, true)
    }

    pub fn fund_transfer(
        &mut self,
        fund_transfer: &types::XTPFundTransferReq,
        session_id: u64,
    ) -> Fallible<i64> {
        let retc = unsafe {
            TraderApi_FundTransfer(
                self.trader_api,
                &mut fund_transfer.into() as *mut _,
                session_id,
            )
        };
        self.translate_code(retc as i64, false)
    }

    pub fn query_fund_transfer(
        &mut self,
        query_param: &types::XTPQueryFundTransferLogReq,
        session_id: u64,
        request_id: i32,
    ) -> Fallible<i64> {
        let retc = unsafe {
            TraderApi_QueryFundTransfer(
                self.trader_api,
                &mut query_param.into() as *mut _,
                session_id,
                request_id,
            )
        };
        self.translate_code(retc as i64, true)
    }

    pub fn query_etf(
        &mut self,
        query_param: &types::XTPQueryETFBaseReq,
        session_id: u64,
        request_id: i32,
    ) -> Fallible<i64> {
        let retc = unsafe {
            TraderApi_QueryETF(
                self.trader_api,
                &mut query_param.into(),
                session_id,
                request_id,
            )
        };
        self.translate_code(retc as i64, true)
    }

    pub fn query_etf_ticker_basket(
        &mut self,
        query_param: &types::XTPQueryETFComponentReq,
        session_id: u64,
        request_id: i32,
    ) -> Fallible<i64> {
        let retc = unsafe {
            TraderApi_QueryETFTickerBasket(
                self.trader_api,
                &mut query_param.into() as *mut _,
                session_id,
                request_id,
            )
        };
        self.translate_code(retc as i64, true)
    }

    pub fn query_ipo_info_list(&mut self, session_id: u64, request_id: i32) -> Fallible<i64> {
        let retc = unsafe { TraderApi_QueryIPOInfoList(self.trader_api, session_id, request_id) };
        self.translate_code(retc as i64, true)
    }

    pub fn query_ipo_quota_info(&mut self, session_id: u64, request_id: i32) -> Fallible<i64> {
        let retc = unsafe { TraderApi_QueryIPOQuotaInfo(self.trader_api, session_id, request_id) };
        self.translate_code(retc as i64, true)
    }

    pub fn query_option_auction_info(
        &mut self,
        query_param: &types::XTPQueryOptionAuctionInfoReq,
        session_id: u64,
        request_id: i32,
    ) -> Fallible<i64> {
        let retc = unsafe {
            TraderApi_QueryOptionAuctionInfo(
                self.trader_api,
                &mut query_param.into() as *mut _,
                session_id,
                request_id,
            )
        };
        self.translate_code(retc as i64, true)
    }
}

impl TraderApi {
    pub fn new(id: u8, path: &str, log_level: types::XTPLogLevel) -> TraderApi {
        let cpath = CString::new(path);
        let trader_api = unsafe {
            CreateTraderApi(
                id,
                cpath.unwrap().as_c_str().as_ptr(),
                transmute::<_, XTP_LOG_LEVEL>(log_level),
            )
        };

        TraderApi {
            trader_api,
            trader_spi_stub: None,
        }
    }

    fn translate_code(&mut self, code: i64, zero_ok: bool) -> Fallible<i64> {
        if (code == 0) == zero_ok {
            return Ok(code);
        }

        let underlying_error = self.get_api_last_error();
        Err(XTPError::XTPClientError {
            error_id: underlying_error.error_id as i64,
            error_msg: underlying_error.error_msg,
        })?
    }
}

impl Drop for TraderApi {
    fn drop(&mut self) {
        self.release();
        if let Some(spi_stub) = self.trader_spi_stub {
            unsafe { Box::from_raw(spi_stub) };
        }
    }
}

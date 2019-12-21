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
use libc::{c_char, c_void};
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

    pub fn subscribe_public_topic(&mut self, resume_type: types::XTPTeResumeType) {}

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
    ) -> Fallible<()> {
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
            self.translate_code(ret_code as i64)
        }
    }

    pub fn logout(&mut self, session_id: u64) -> Fallible<()> {
        let retc = unsafe { TraderApi_Logout(self.trader_api, session_id) };
        self.translate_code(retc as i64)
    }

    pub fn is_server_restart(&mut self, session_id: u64) -> bool {
        unsafe { TraderApi_IsServerRestart(self.trader_api, session_id) }
    }

    pub fn insert_order(
        &mut self,
        order: &types::XTPOrderInsertInfo,
        session_id: u64,
    ) -> Fallible<()> {
        let mut order = order.into();
        let retc =
            unsafe { TraderApi_InsertOrder(self.trader_api, &mut order as *mut _, session_id) };
        self.translate_code(retc as i64)
    }

    pub fn cancel_order(&mut self, order_xtp_id: u64, session_id: u64) -> Fallible<()> {
        let retc = unsafe { TraderApi_CancelOrder(self.trader_api, order_xtp_id, session_id) };
        self.translate_code(retc as i64)
    }

    // pub fn QueryOrderByXTPID(
    //     &mut self,
    //     order_xtp_id: u64,
    //     session_id: u64,
    //     request_id: i32,
    // ) -> i32 {
    //     TraderApi_QueryOrderByXTPID
    // }
    // pub fn QueryOrders(
    //     &mut self,
    //     query_param: &XTPQueryOrderReq,
    //     session_id: u64,
    //     request_id: i32,
    // ) -> i32 {
    //     TraderApi_QueryOrders
    // }
    // pub fn QueryOrdersByPage(
    //     &mut self,
    //     query_param: &XTPQueryOrderByPageReq,
    //     session_id: u64,
    //     request_id: i32,
    // ) -> i32 {
    //     TraderApi_QueryOrdersByPage
    // }
    // pub fn QueryTradesByXTPID(
    //     &mut self,
    //     order_xtp_id: u64,
    //     session_id: u64,
    //     request_id: i32,
    // ) -> i32 {
    //     TraderApi_QueryTradesByXTPID
    // }
    // pub fn QueryTrades(
    //     &mut self,
    //     query_param: &XTPQueryTraderReq,
    //     session_id: u64,
    //     request_id: i32,
    // ) -> i32 {
    //     TraderApi_QueryTrades
    // }
    // pub fn QueryTradesByPage(
    //     &mut self,
    //     query_param: &XTPQueryTraderByPageReq,
    //     session_id: u64,
    //     request_id: i32,
    // ) -> i32 {
    //     TraderApi_QueryTradesByPage
    // }
    // pub fn QueryPosition(&mut self, ticker: &str, session_id: u64, request_id: i32) -> i32 {
    //     TraderApi_QueryPosition
    // }
    // pub fn QueryAsset(&mut self, session_id: u64, request_id: i32) -> i32 {
    //     TraderApi_QueryAsset
    // }
    // pub fn QueryStructuredFund(
    //     &mut self,
    //     query_param: &XTPQueryStructuredFundInfoReq,
    //     session_id: u64,
    //     request_id: i32,
    // ) -> i32 {
    //     TraderApi_QueryStructuredFund
    // }
    // pub fn FundTransfer(&mut self, fund_transfer: &XTPFundTransferReq, session_id: u64) -> u64 {
    //     TraderApi_FundTransfer
    // }
    // pub fn QueryFundTransfer(
    //     &mut self,
    //     query_param: XTPQueryFundTransferLogReq,
    //     session_id: u64,
    //     request_id: i32,
    // ) -> i32 {
    //     TraderApi_QueryFundTransfer
    // }
    // pub fn QueryETF(
    //     &mut self,
    //     query_param: XTPQueryETFBaseReq,
    //     session_id: u64,
    //     request_id: i32,
    // ) -> i32 {
    //     TraderApi_QueryETF
    // }
    // pub fn QueryETFTickerBasket(
    //     &mut self,
    //     query_param: &XTPQueryETFComponentReq,
    //     session_id: u64,
    //     request_id: i32,
    // ) -> i32 {
    //     TraderApi_QueryETFTickerBasket
    // }
    // pub fn QueryIPOInfoList(&mut self, session_id: u64, request_id: i32) -> i32 {
    //     TraderApi_QueryIPOInfoList
    // }
    // pub fn QueryIPOQuotaInfo(&mut self, session_id: u64, request_id: i32) -> i32 {
    //     TraderApi_QueryIPOQuotaInfo
    // }
    // pub fn QueryOptionAuctionInfo(
    //     &mut self,
    //     query_param: XTPQueryOptionAuctionInfoReq,
    //     session_id: u64,
    //     request_id: i32,
    // ) -> i32 {
    //     TraderApi_QueryOptionAuctionInfo
    // }
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

    fn translate_code(&mut self, code: i64) -> Fallible<()> {
        if code != 0 {
            let underlying_error = self.get_api_last_error();
            Err(XTPError::XTPClientError {
                error_id: underlying_error.error_id as i64,
                error_msg: underlying_error.error_msg,
            })?;
        }
        Ok(())
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

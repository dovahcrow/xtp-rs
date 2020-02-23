use crate::errors::XTPError;
use crate::quote_spi::QuoteSpi;
use crate::sys::{
    CreateQuoteApi, QuoteApi_GetApiLastError, QuoteApi_GetApiVersion, QuoteApi_GetTradingDay,
    QuoteApi_Login, QuoteApi_Logout, QuoteApi_QueryAllTickers, QuoteApi_QueryAllTickersPriceInfo,
    QuoteApi_QueryTickersPriceInfo, QuoteApi_RegisterSpi, QuoteApi_Release,
    QuoteApi_SetHeartBeatInterval, QuoteApi_SetUDPBufferSize, QuoteApi_SubscribeAllMarketData,
    QuoteApi_SubscribeAllOptionMarketData, QuoteApi_SubscribeAllOptionOrderBook,
    QuoteApi_SubscribeAllOptionTickByTick, QuoteApi_SubscribeAllOrderBook,
    QuoteApi_SubscribeAllTickByTick, QuoteApi_SubscribeMarketData, QuoteApi_SubscribeOrderBook,
    QuoteApi_SubscribeTickByTick, QuoteApi_UnSubscribeAllMarketData,
    QuoteApi_UnSubscribeAllOptionMarketData, QuoteApi_UnSubscribeAllOptionOrderBook,
    QuoteApi_UnSubscribeAllOptionTickByTick, QuoteApi_UnSubscribeAllOrderBook,
    QuoteApi_UnSubscribeAllTickByTick, QuoteApi_UnSubscribeMarketData,
    QuoteApi_UnSubscribeOrderBook, QuoteApi_UnSubscribeTickByTick, QuoteSpiStub,
    QuoteSpiStub_Destructor, XTP_API_QuoteApi, XTP_API_QuoteSpi, XTP_EXCHANGE_TYPE, XTP_LOG_LEVEL,
};
use crate::types;
use crate::types::FromRaw;
use failure::Fallible;
use libc::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::net::SocketAddrV4;

unsafe impl Send for QuoteApi {}
unsafe impl Sync for QuoteApi {}

pub struct QuoteApi {
    quote_api: *mut XTP_API_QuoteApi,
    quote_spi_stub: Option<*mut QuoteSpiStub>, // Free the stub after we freed XTP_API_QuoteApi in drop()
}

impl QuoteApi {
    pub fn new(id: u8, path: &str, log_level: types::XTPLogLevel) -> QuoteApi {
        let cpath = CString::new(path);
        let quote_api = unsafe {
            CreateQuoteApi(
                id,
                cpath.unwrap().as_c_str().as_ptr(),
                transmute::<_, XTP_LOG_LEVEL>(log_level),
            )
        };

        QuoteApi {
            quote_api,
            quote_spi_stub: None,
        }
    }
    fn translate_code(&mut self, code: i32) -> Fallible<()> {
        if code != 0 {
            let underlying_error = self.get_api_last_error();
            return Err(XTPError::XTPClientError {
                error_id: underlying_error.error_id as i64,
                error_msg: underlying_error.error_msg,
            }
            .into());
        }
        Ok(())
    }

    fn call_by_tickers(
        &mut self,
        func: unsafe extern "C" fn(
            self_: *mut XTP_API_QuoteApi,
            ticker: *mut *mut c_char,
            count: i32,
            exchange_id: XTP_EXCHANGE_TYPE,
        ) -> i32,
        tickers: &[&str],
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        let mut cstring_tickers: Vec<_> = tickers
            .iter()
            .map(|ticker| CString::new(*ticker).unwrap().into_raw())
            .collect();

        let ret_code = unsafe {
            func(
                self.quote_api,
                cstring_tickers.as_mut_ptr(),
                cstring_tickers.len() as i32,
                exchange_id.into(),
            )
        };

        for ticker_ptr in cstring_tickers {
            unsafe { CString::from_raw(ticker_ptr) };
        }
        self.translate_code(ret_code)
    }

    fn call_by_exchange(
        &mut self,
        func: unsafe extern "C" fn(
            self_: *mut XTP_API_QuoteApi,
            exchange_id: XTP_EXCHANGE_TYPE,
        ) -> i32,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        let ret_code = unsafe { func(self.quote_api, exchange_id.into()) };
        self.translate_code(ret_code)
    }
}

impl QuoteApi {
    fn release(&mut self) {
        unsafe { QuoteApi_Release(self.quote_api) };
    }

    pub fn get_trading_day(&mut self) -> &str {
        let ptr = unsafe { QuoteApi_GetTradingDay(self.quote_api) }; // The string is freed by them
        unsafe { CStr::from_ptr(ptr) }.to_str().unwrap()
    }

    pub fn get_api_version(&mut self) -> &CStr {
        let ptr = unsafe { QuoteApi_GetApiVersion(self.quote_api) }; // The string is freed by them
        unsafe { CStr::from_ptr(ptr) }
    }

    pub fn get_api_last_error(&mut self) -> types::XTPRspInfoStruct {
        unsafe { types::XTPRspInfoStruct::from_raw(&*QuoteApi_GetApiLastError(self.quote_api)) }
    }

    pub fn set_udp_buffer_size(&mut self, buffer_size: u32) {
        unsafe { QuoteApi_SetUDPBufferSize(self.quote_api, buffer_size) }
    }

    pub fn register_spi<T: QuoteSpi>(&mut self, spi: T) {
        let trait_object_box: Box<Box<dyn QuoteSpi>> = Box::new(Box::new(spi));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<dyn QuoteSpi> as *mut c_void;

        let quote_spi_stub = unsafe { QuoteSpiStub::new(trait_object_pointer) };

        let ptr = Box::into_raw(Box::new(quote_spi_stub));
        self.quote_spi_stub = Some(ptr);
        unsafe { QuoteApi_RegisterSpi(self.quote_api, ptr as *mut XTP_API_QuoteSpi) };
    }

    pub fn set_heart_beat_interval(&mut self, interval: u32) {
        unsafe { QuoteApi_SetHeartBeatInterval(self.quote_api, interval) }
    }

    pub fn subscribe_market_data(
        &mut self,
        tickers: &[&str],
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_tickers(QuoteApi_SubscribeMarketData, tickers, exchange_id)
    }

    pub fn unsubscribe_market_data(
        &mut self,
        tickers: &[&str],
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_tickers(QuoteApi_UnSubscribeMarketData, tickers, exchange_id)
    }

    pub fn subscribe_order_book(
        &mut self,
        tickers: &[&str],
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_tickers(QuoteApi_SubscribeOrderBook, tickers, exchange_id)
    }

    pub fn unsubscribe_order_book(
        &mut self,
        tickers: &[&str],
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_tickers(QuoteApi_UnSubscribeOrderBook, tickers, exchange_id)
    }

    pub fn subscribe_tick_by_tick(
        &mut self,
        tickers: &[&str],
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_tickers(QuoteApi_SubscribeTickByTick, tickers, exchange_id)
    }

    pub fn unsubscribe_tick_by_tick(
        &mut self,
        tickers: &[&str],
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_tickers(QuoteApi_UnSubscribeTickByTick, tickers, exchange_id)
    }

    pub fn subscribe_all_market_data(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_SubscribeAllMarketData, exchange_id)
    }

    pub fn unsubscribe_all_market_data(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_UnSubscribeAllMarketData, exchange_id)
    }

    pub fn subscribe_all_order_book(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_SubscribeAllOrderBook, exchange_id)
    }

    pub fn unsubscribe_all_order_book(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_UnSubscribeAllOrderBook, exchange_id)
    }

    pub fn subscribe_all_tick_by_tick(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_SubscribeAllTickByTick, exchange_id)
    }

    pub fn unsubscribe_all_tick_by_tick(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_UnSubscribeAllTickByTick, exchange_id)
    }

    pub fn login(
        &mut self,
        server_addr: SocketAddrV4,
        username: &str,
        password: &str,
        sock_type: types::XTPProtocolType,
    ) -> Fallible<()> {
        let ip = CString::new(format!("{}", server_addr.ip())).unwrap();
        let username = CString::new(username).unwrap();
        let password = CString::new(password).unwrap();
        let ret_code = unsafe {
            QuoteApi_Login(
                self.quote_api,
                ip.as_ptr(),
                server_addr.port() as i32,
                username.as_ptr(),
                password.as_ptr(),
                sock_type.into(),
            )
        };
        match ret_code {
            -2 => Err(XTPError::DuplicatedLogin.into()),
            _ => self.translate_code(ret_code),
        }
    }

    pub fn logout(&mut self) -> Fallible<()> {
        let ret_code = unsafe { QuoteApi_Logout(self.quote_api) };
        self.translate_code(ret_code)
    }

    pub fn query_all_tickers(&mut self, exchange_id: types::XTPExchangeType) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_QueryAllTickers, exchange_id)
    }

    pub fn query_tickers_price_info(
        &mut self,
        tickers: &[&str],
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_tickers(QuoteApi_QueryTickersPriceInfo, tickers, exchange_id)
    }

    pub fn query_all_tickers_price_info(&mut self) -> Fallible<()> {
        let code = unsafe { QuoteApi_QueryAllTickersPriceInfo(self.quote_api) };
        self.translate_code(code)
    }

    pub fn subscribe_all_option_market_data(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_SubscribeAllOptionMarketData, exchange_id)
    }

    pub fn unsubscribe_all_option_market_data(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_UnSubscribeAllOptionMarketData, exchange_id)
    }

    pub fn subscribe_all_option_order_book(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_SubscribeAllOptionOrderBook, exchange_id)
    }

    pub fn unsubscribe_all_option_order_book(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_UnSubscribeAllOptionOrderBook, exchange_id)
    }

    pub fn subscribe_all_option_tick_by_tick(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_SubscribeAllOptionTickByTick, exchange_id)
    }

    pub fn unsubscribe_all_option_tick_by_tick(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        self.call_by_exchange(QuoteApi_UnSubscribeAllOptionTickByTick, exchange_id)
    }
}

impl Drop for QuoteApi {
    fn drop(&mut self) {
        self.release();
        if let Some(spi_stub) = self.quote_spi_stub {
            unsafe { QuoteSpiStub_Destructor(spi_stub) };
        }
    }
}

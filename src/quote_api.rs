use crate::errors::XTPError;
use crate::quote_spi::QuoteSpi;
use crate::sys::{
    CreateQuoteApi, QuoteApi_GetApiLastError, QuoteApi_GetApiVersion, QuoteApi_GetTradingDay,
    QuoteApi_Login, QuoteApi_Logout, QuoteApi_RegisterSpi, QuoteApi_Release,
    QuoteApi_SubscribeAllOrderBook, QuoteSpiStub, XTP_API_QuoteApi, XTP_API_QuoteSpi, XTPRI,
    XTP_EXCHANGE_TYPE, XTP_LOG_LEVEL, XTP_PROTOCOL_TYPE,
};
use crate::types;
use failure::Fallible;
use libc::c_void;
use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::net::SocketAddrV4;

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

    pub fn register_spi<T: QuoteSpi>(&mut self, spi: T) {
        let trait_object_box: Box<Box<dyn QuoteSpi>> = Box::new(Box::new(spi));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<dyn QuoteSpi> as *mut c_void;

        let quote_spi_stub = unsafe { QuoteSpiStub::new(trait_object_pointer) };

        let ptr = Box::into_raw(Box::new(quote_spi_stub));
        self.quote_spi_stub = Some(ptr);
        unsafe { QuoteApi_RegisterSpi(self.quote_api, ptr as *mut XTP_API_QuoteSpi) };
    }

    pub fn subscribe_all_order_book(
        &mut self,
        exchange_id: types::XTPExchangeType,
    ) -> Fallible<()> {
        let ret_code = unsafe {
            QuoteApi_SubscribeAllOrderBook(
                self.quote_api,
                transmute::<_, XTP_EXCHANGE_TYPE>(exchange_id),
            )
        };
        self.translate_code(ret_code)
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
                transmute::<_, XTP_PROTOCOL_TYPE>(sock_type),
            )
        };
        self.translate_code(ret_code)
    }

    pub fn logout(&mut self) -> Fallible<()> {
        let ret_code = unsafe { QuoteApi_Logout(self.quote_api) };
        self.translate_code(ret_code)
    }

    fn translate_code(&mut self, code: i32) -> Fallible<()> {
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

impl Drop for QuoteApi {
    fn drop(&mut self) {
        self.release();
        if let Some(spi_stub) = self.quote_spi_stub {
            unsafe { Box::from_raw(spi_stub) };
        }
    }
}

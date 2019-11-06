use crate::quote_spi::QuoteSpi;
use crate::types;
use libc::c_void;
use std::ffi::{CStr, CString};
use std::mem::transmute;
use xtp_sys::{
    CreateQuoteApi, QuoteApi_GetApiLastError, QuoteApi_GetApiVersion, QuoteApi_GetTradingDay,
    QuoteApi_RegisterSpi, QuoteApi_Release, QuoteSpiStub, XTP_API_QuoteApi, XTP_API_QuoteSpi,
    XTPRI, XTP_LOG_LEVEL,
};

pub struct QuoteApi {
    quote_api: *mut XTP_API_QuoteApi,
    quote_spi_stub: Option<*mut XTP_API_QuoteSpi>,
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

    pub fn get_trading_day(&self) -> &str {
        let ptr = unsafe { QuoteApi_GetTradingDay(self.quote_api) }; // The string is freed by them
        unsafe { CStr::from_ptr(ptr) }.to_str().unwrap()
    }

    pub fn get_api_version(&self) -> &CStr {
        let ptr = unsafe { QuoteApi_GetApiVersion(self.quote_api) }; // The string is freed by them
        unsafe { CStr::from_ptr(ptr) }
    }

    pub fn get_api_last_error(&self) -> types::XTPRspInfoStruct {
        let XTPRI {
            error_id,
            error_msg,
        } = unsafe { &*QuoteApi_GetApiLastError(self.quote_api) };
        let error_msg = unsafe { CStr::from_ptr(error_msg as *const [i8] as *const i8) };
        types::XTPRspInfoStruct {
            error_id: *error_id,
            error_msg: error_msg.to_owned().to_string_lossy().to_string(),
        }
    }

    pub fn register_spi<T: QuoteSpi>(&mut self, spi: T) {
        let trait_object_box: Box<Box<dyn QuoteSpi>> = Box::new(Box::new(spi));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<dyn QuoteSpi> as *mut c_void;

        let quote_spi_stub = unsafe { QuoteSpiStub::new(trait_object_pointer) };

        let ptr = Box::into_raw(Box::new(quote_spi_stub)) as *mut XTP_API_QuoteSpi;
        self.quote_spi_stub = Some(ptr);
        unsafe { QuoteApi_RegisterSpi(self.quote_api, ptr) };
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

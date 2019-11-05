use crate::quote_spi::QuoteSpi;
use libc::c_void;
use std::ffi::CString;
use xtp_sys::{
    CreateQuoteApi, QuoteApi_RegisterSpi, QuoteSpiStub, XTP_API_QuoteApi, XTP_API_QuoteSpi,
    XTP_LOG_LEVEL,
};

pub struct QuoteApi {
    quote_api: *mut XTP_API_QuoteApi,
}

impl QuoteApi {
    pub fn new(id: u8, path: &str, log_level: XTP_LOG_LEVEL) -> QuoteApi {
        let cpath = CString::new(path);
        let quote_api =
            unsafe { CreateQuoteApi(id, cpath.unwrap().as_c_str().as_ptr(), log_level) };

        QuoteApi { quote_api }
    }

    pub fn register_spi<T: QuoteSpi>(&self, spi: T) {
        let trait_object_box: Box<Box<dyn QuoteSpi>> = Box::new(Box::new(spi));
        let trait_object_pointer = Box::into_raw(trait_object_box) as *mut c_void;
        let quote_spi_stub = unsafe { QuoteSpiStub::new(trait_object_pointer) };

        let boxed = Box::into_raw(Box::new(quote_spi_stub)) as *mut XTP_API_QuoteSpi;
        unsafe { QuoteApi_RegisterSpi(self.quote_api, boxed) };
    }
}

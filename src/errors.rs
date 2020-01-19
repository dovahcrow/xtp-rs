use failure::Fail;

/// Error type for this library.
#[derive(Debug, Fail, Clone)]
pub enum XTPError {
    /// Error message from underlying xtp library
    #[fail(display = "XTP client error {}: {}", error_id, error_msg)]
    XTPClientError { error_id: i64, error_msg: String },

    /// Error when converting from C enum to Rust failed.
    #[fail(display = "Cannot convert from {}, integer violation", _0)]
    RawTypeConvertionError(String),
}

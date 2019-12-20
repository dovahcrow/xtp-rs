use failure::Fail;

#[derive(Debug, Fail, Clone)]
pub enum XTPError {
    #[fail(display = "XTP client error {}: {}", error_id, error_msg)]
    XTPClientError { error_id: i64, error_msg: String },
    #[fail(display = "Cannot convert from {}, integer violation", _0)]
    RawTypeConvertionError(String),
}

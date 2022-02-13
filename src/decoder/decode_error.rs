use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("decode error: {msg}\n\tat {location}")]
    Error { msg: String, location: String },
}

#[macro_export]
macro_rules! decode_err {
    ($msg:expr $(,)?) => {
        $crate::decoder::decode_error::DecodeError::Error {
            msg: format!($msg),
            location: format!("{}:{}:{}", file!(), line!(), column!()),
        }
    };
}

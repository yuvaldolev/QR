use std::result;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid Alphanumeric encoding table key: '{0}'")]
    InvalidAlphanumericEncodingTableKey(char),

    #[error("data too long: length={0}")]
    DataTooLong(usize),
}

pub type Result<T> = result::Result<T, Error>;

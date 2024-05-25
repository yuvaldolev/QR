use std::result;

use aws_sdk_sqs::{error::SdkError, operation::send_message::SendMessageError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid Alphanumeric encoding table key: '{0}'")]
    InvalidAlphanumericEncodingTableKey(char),

    #[error("data too long to be encoded in a single QR code")]
    DataTooLong,

    #[error("failed serializing request '{0}' to JSON")]
    SerializeRequest(#[source] serde_json::Error, String),

    #[error("failed sending message '{0}' to SQS '{1}'")]
    SendSqsMessage(#[source] SdkError<SendMessageError>, String, String),
}

pub type Result<T> = result::Result<T, Error>;

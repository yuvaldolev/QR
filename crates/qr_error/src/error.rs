use std::result;

use aws_sdk_sqs::{error::SdkError, operation::send_message::SendMessageError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid Alphanumeric encoding table key: '{0}'")]
    InvalidAlphanumericEncodingTableKey(char),

    #[error("data too long to be encoded in a single QR code")]
    DataTooLong,

    #[error("request body is missing")]
    MissingRequestBody,

    #[error("failed deserializing request '{1}' to JSON")]
    DeserializeRequest(#[source] serde_json::Error, String),

    #[error("failed serializing response to JSON")]
    SerializeResponse(#[source] serde_json::Error),

    #[error("failed serializing SQS message to JSON")]
    SerializeQueueMessage(#[source] serde_json::Error),

    #[error("failed sending message '{1}' to SQS '{2}'")]
    SendSqsMessage(#[source] SdkError<SendMessageError>, String, String),
}

pub type Result<T> = result::Result<T, Error>;

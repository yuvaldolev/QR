use std::result;

use aws_sdk_dynamodb::operation::{
    delete_item::DeleteItemError, put_item::PutItemError, query::QueryError,
};
use aws_sdk_sqs::operation::send_message::SendMessageError;

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
    SendSqsMessage(
        #[source] aws_sdk_sqs::error::SdkError<SendMessageError>,
        String,
        String,
    ),

    #[error("no connection ID found in WebSocket request")]
    NoWebSocketConnectionId,

    #[error("expected a request")]
    ExpectedRequest,

    #[error("unexpected request")]
    UnexpectedRequest,

    #[error("WebSocket request is not an object: '{0}'")]
    WebSocketRequestNotAnObject(String),

    #[error("WebSocket request is missing data: '{0}'")]
    WebSocketRequestMissingData(String),

    #[error("failed logging result WebSocket in DynamoDB: request_id='{1}', connection_id='{2}'")]
    LogResultWebSocketInDynamoDB(
        #[source] aws_sdk_dynamodb::error::SdkError<PutItemError>,
        String,
        String,
    ),

    #[error("failed querying result WebSocket from DynamoDB: connection_id='{1}'")]
    QueryResultWebSocketFromDynamoDB(
        #[source] aws_sdk_dynamodb::error::SdkError<QueryError>,
        String,
    ),

    #[error("unexpected result WebSocket count: expected 1, got '{1}' for connection_id='{0}'")]
    UnexpectedResultWebSocketCount(String, i32),

    #[error("empty result WebSocket query items for connection_id='{0}'")]
    EmptyResultWebSocketQueryItems(String),

    #[error("missing result WebSocket request ID for connection_id='{0}'")]
    MissingResultWebSocketRequestId(String),

    #[error(
        "unexpected result WebSocket request ID type for connection_id='{0}', expected String"
    )]
    UnexpectedResultWebSocketRequestIdType(String),

    #[error("failed deleting result WebSocket from DynamoDB: connection_id='{1}'")]
    DeleteResultWebSocketFromDynamoDB(
        #[source] aws_sdk_dynamodb::error::SdkError<DeleteItemError>,
        String,
    ),
}

pub type Result<T> = result::Result<T, Error>;

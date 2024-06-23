use std::result;

use aws_sdk_apigatewaymanagement::operation::post_to_connection::PostToConnectionError;
use aws_sdk_dynamodb::operation::{
    delete_item::DeleteItemError, get_item::GetItemError, put_item::PutItemError, query::QueryError,
};
use aws_sdk_sqs::operation::{
    change_message_visibility::ChangeMessageVisibilityError, send_message::SendMessageError,
};
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid Alphanumeric encoding table key: '{0}'")]
    InvalidAlphanumericEncodingTableKey(char),

    #[error("data too long to be encoded in a single QR code")]
    DataTooLong,

    #[error("request body is missing")]
    MissingRequestBody,

    #[error("failed deserializing API Gateway request '{1}' from JSON")]
    DeserializeApiGatewayRequest(#[source] serde_json::Error, String),

    #[error("failed serializing API Gateway response to JSON")]
    SerializeApiGatewayResponse(#[source] serde_json::Error),

    #[error("failed deserializing WebSocket request '{1}' from JSON")]
    DeserializeWebSocketRequest(#[source] serde_json::Error, String),

    #[error("failed serializing WebSocket response to JSON")]
    SerializeWebSocketResponse(#[source] serde_json::Error),

    #[error("failed deserializing SQS input message '{1}' from JSON")]
    DeserializeSqsInputMessage(#[source] serde_json::Error, Value),

    #[error("failed serializing SQS output message to JSON")]
    SerializeSqsOutputMessage(#[source] serde_json::Error),

    #[error("failed serializing SQS message to JSON")]
    SerializeQueueMessage(#[source] serde_json::Error),

    #[error("failed sending message '{1}' to SQS '{2}'")]
    SendSqsMessage(
        #[source] aws_sdk_sqs::error::SdkError<SendMessageError>,
        String,
        String,
    ),

    #[error("failed changing visibility timeout of message '{1}' to '{2}'")]
    ChangeMessageVisibilityTimeout(
        #[source] aws_sdk_sqs::error::SdkError<ChangeMessageVisibilityError>,
        String,
        i32,
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

    #[error("failed to handle message: {0}")]
    HandleMessage(String),

    #[error("failed to get result WebSocket from DynamoDB: request_id='{0}'")]
    GetResultWebSocketFromDynamoDB(
        #[source] aws_sdk_dynamodb::error::SdkError<GetItemError>,
        String,
    ),

    #[error("no result WebSocket found in DynamoDB for request_id='{0}'")]
    NoResultWebSocketFound(String),

    #[error("missing result WebSocket connection ID for request_id='{0}'")]
    MissingResultWebSocketConnectionId(String),

    #[error(
        "unexpected result WebSocket connection ID type for request_id='{0}', expected String"
    )]
    UnexpectedResultWebSocketConnectionIdType(String),

    #[error("failed to serialize encode result output message to JSON")]
    SerializeEncodeResultOutputMessage(#[source] serde_json::Error),

    #[error("failed to post encode result output message '{1}' to WebSocket: connection_id='{2}'")]
    PostEncodeResultOutputMessageToWebSocket(
        #[source] aws_sdk_apigatewaymanagement::error::SdkError<PostToConnectionError>,
        String,
        String,
    ),

    #[error("missing message ID")]
    MissingMessageId,

    #[error("message '{0}' is missing receipt handle")]
    MissingMessageReceiptHandle(String),
}

pub type Result<T> = result::Result<T, Error>;

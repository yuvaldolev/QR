use aws_sdk_apigatewaymanagement::primitives::Blob;
use aws_sdk_dynamodb::types::AttributeValue;
use lambda_runtime::{tracing, Context};
use qr_aws::functions::sqs_handler::HandleMessage;

use crate::{
    qr_encode_result_input::QrEncodeResultInput, qr_encode_result_output::QrEncodeResultOutput,
};

const NO_WEB_SOCKET_IN_DB_ERROR_VISIBILITY_TIMEOUT: i32 = 1;

pub struct MessageHandler {
    table_name: String,
    dynamodb_client: aws_sdk_dynamodb::Client,
    source_queue_url: String,
    sqs_client: aws_sdk_sqs::Client,
    api_gateway_management_client: aws_sdk_apigatewaymanagement::Client,
}

impl MessageHandler {
    pub fn new(
        table_name: String,
        dynamodb_client: aws_sdk_dynamodb::Client,
        source_queue_url: String,
        sqs_client: aws_sdk_sqs::Client,
        api_gateway_management_client: aws_sdk_apigatewaymanagement::Client,
    ) -> Self {
        Self {
            table_name,
            dynamodb_client,
            source_queue_url,
            sqs_client,
            api_gateway_management_client,
        }
    }
}

#[async_trait::async_trait]
impl HandleMessage for MessageHandler {
    type Message = QrEncodeResultInput;

    async fn handle_message(
        &self,
        _context: &Context,
        id: &str,
        receipt_handle: &str,
        message: Self::Message,
    ) -> qr_error::Result<()> {
        tracing::info!("Handling message: id='{id}', receipt_handle='{receipt_handle}'");

        let get_item_output = self
            .dynamodb_client
            .get_item()
            .table_name(&self.table_name)
            .key("requestId", AttributeValue::S(message.get_id().to_owned()))
            .send()
            .await
            .map_err(|e| {
                qr_error::Error::GetResultWebSocketFromDynamoDB(e, message.get_id().to_owned())
            })?;

        let item = match get_item_output.item {
            Some(item) => item,
            None => {
                tracing::warn!(
                "No result WebSocket found in DynamoDB for request_id='{}', changing visibility timeout of message '{}' to {}s",
                message.get_id(),
                id,
                NO_WEB_SOCKET_IN_DB_ERROR_VISIBILITY_TIMEOUT,
            );
                self.sqs_client
                    .change_message_visibility()
                    .queue_url(&self.source_queue_url)
                    .receipt_handle(receipt_handle)
                    .visibility_timeout(NO_WEB_SOCKET_IN_DB_ERROR_VISIBILITY_TIMEOUT)
                    .send()
                    .await
                    .map_err(|e| {
                        qr_error::Error::ChangeMessageVisibilityTimeout(
                            e,
                            id.to_owned(),
                            NO_WEB_SOCKET_IN_DB_ERROR_VISIBILITY_TIMEOUT,
                        )
                    })?;

                return Err(qr_error::Error::NoResultWebSocketFound(
                    message.get_id().to_owned(),
                ));
            }
        };

        let connection_id = item
            .get("webSocketId")
            .ok_or_else(|| {
                qr_error::Error::MissingResultWebSocketConnectionId(message.get_id().to_owned())
            })?
            .as_s()
            .map_err(|_| {
                qr_error::Error::UnexpectedResultWebSocketConnectionIdType(
                    message.get_id().to_owned(),
                )
            })?;

        tracing::info!(
            "Found result WebSocket for request_id='{}': connection_id='{}'",
            message.get_id(),
            connection_id,
        );

        let output_message = QrEncodeResultOutput::new(message.get_data().to_owned());
        let output_message_json = serde_json::to_string(&output_message)
            .map_err(qr_error::Error::SerializeEncodeResultOutputMessage)?;
        tracing::trace!("Serialized output message to JSON: '{output_message_json}'");

        self.api_gateway_management_client
            .post_to_connection()
            .connection_id(connection_id)
            .data(Blob::new(output_message_json.as_bytes()))
            .send()
            .await
            .map_err(|e| {
                qr_error::Error::PostEncodeResultOutputMessageToWebSocket(
                    e,
                    output_message_json.clone(),
                    connection_id.to_owned(),
                )
            })?;

        tracing::info!("Posted output message to WebSocket");

        Ok(())
    }
}

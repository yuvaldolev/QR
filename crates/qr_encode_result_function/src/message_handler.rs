use aws_sdk_apigatewaymanagement::primitives::Blob;
use aws_sdk_dynamodb::types::AttributeValue;
use lambda_runtime::{tracing, Context};
use qr_aws::functions::sqs_handler::HandleMessage;

use crate::{
    qr_encode_result_input::QrEncodeResultInput, qr_encode_result_output::QrEncodeResultOutput,
};

pub struct MessageHandler {
    table_name: String,
    dynamodb_client: aws_sdk_dynamodb::Client,
    api_gateway_management_client: aws_sdk_apigatewaymanagement::Client,
}

impl MessageHandler {
    pub fn new(
        table_name: String,
        dynamodb_client: aws_sdk_dynamodb::Client,
        api_gateway_management_client: aws_sdk_apigatewaymanagement::Client,
    ) -> Self {
        Self {
            table_name,
            dynamodb_client,
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
        message: Self::Message,
    ) -> qr_error::Result<()> {
        tracing::info!("Handling message '{id}'");

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

        let item = get_item_output
            .item
            .ok_or_else(|| qr_error::Error::NoResultWebSocketFound(message.get_id().to_owned()))?;

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

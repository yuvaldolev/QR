use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use lambda_runtime::{tracing, Context};
use qr_aws::functions::web_socket_handler::HandleEvent;

use crate::qr_result_monitor_request::QrResultMonitorRequest;

pub struct EventHandler {
    table_name: String,
    dynamodb_client: aws_sdk_dynamodb::Client,
}

impl EventHandler {
    pub fn new(aws_configuration: &SdkConfig, table_name: String) -> Self {
        Self {
            table_name,
            dynamodb_client: aws_sdk_dynamodb::Client::new(aws_configuration),
        }
    }
}

#[async_trait::async_trait]
impl HandleEvent for EventHandler {
    type Request = QrResultMonitorRequest;
    type Response = String;

    async fn handle_event(
        &self,
        _context: &Context,
        connection_id: &str,
        request: Option<&Self::Request>,
    ) -> qr_error::Result<Self::Response> {
        let request = request.ok_or(qr_error::Error::ExpectedRequest)?;

        tracing::info!(
            "Handling monitor event: request_id='{}', connection_id='{}'",
            request.get_request_id(),
            connection_id
        );

        self.dynamodb_client
            .put_item()
            .table_name(self.table_name.clone())
            .item(
                "requestId",
                AttributeValue::S(request.get_request_id().to_owned()),
            )
            .item("webSocketId", AttributeValue::S(connection_id.to_owned()))
            .send()
            .await
            .map_err(|e| {
                qr_error::Error::LogResultWebSocketInDynamoDB(
                    e,
                    request.get_request_id().to_owned(),
                    connection_id.to_owned(),
                )
            })?;
        tracing::info!("Logged result WebSocket in DynamoDB");

        Ok(String::from("Successfully logged connection in DynamoDB"))
    }
}

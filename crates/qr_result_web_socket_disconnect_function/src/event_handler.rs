use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use lambda_runtime::{tracing, Context};
use qr_aws::functions::web_socket_handler::HandleEvent;

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
    type Request = ();
    type Response = String;

    async fn handle_event(
        &self,
        _context: &Context,
        connection_id: &str,
        request: Option<&Self::Request>,
    ) -> qr_error::Result<Self::Response> {
        request.map_or(Ok(()), |_| Err(qr_error::Error::UnexpectedRequest))?;

        tracing::info!("Handling disconnect event: connection_id='{connection_id}'");

        let query_output = self
            .dynamodb_client
            .query()
            .table_name(self.table_name.clone())
            .index_name("WebSocketIdIndex")
            .key_condition_expression("webSocketId = :connection_id")
            .expression_attribute_values(
                ":connection_id",
                AttributeValue::S(connection_id.to_owned()),
            )
            .send()
            .await
            .map_err(|e| {
                qr_error::Error::QueryResultWebSocketFromDynamoDB(e, connection_id.to_owned())
            })?;
        if 1 != query_output.count {
            return Err(qr_error::Error::UnexpectedResultWebSocketCount(
                connection_id.to_owned(),
                query_output.count,
            ));
        }

        let Some(items) = query_output.items else {
            return Err(qr_error::Error::EmptyResultWebSocketQueryItems(
                connection_id.to_owned(),
            ));
        };

        let item = &items[0];

        let Some(request_id) = item.get("requestId") else {
            return Err(qr_error::Error::MissingResultWebSocketRequestId(
                connection_id.to_owned(),
            ));
        };

        self.dynamodb_client
            .delete_item()
            .table_name(self.table_name.clone())
            .key("requestId", request_id.clone())
            .send()
            .await
            .map_err(|e| {
                qr_error::Error::DeleteResultWebSocketFromDynamoDB(e, connection_id.to_owned())
            })?;

        tracing::info!("Deleted result WebSocket from DynamoDB");

        Ok(String::from("Disconnected!"))
    }
}

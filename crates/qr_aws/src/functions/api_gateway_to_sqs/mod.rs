mod handle_event;

pub use handle_event::HandleEvent;

use aws_config::SdkConfig;
use aws_lambda_events::{
    apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse},
    encodings::Body,
    http::HeaderMap,
};
use lambda_runtime::{tracing, LambdaEvent};

pub struct Function<EventHandler> {
    queue_url: String,
    event_handler: EventHandler,
    sqs_client: aws_sdk_sqs::Client,
}

impl<EventHandler> Function<EventHandler>
where
    EventHandler: HandleEvent,
{
    pub fn new(
        aws_configuration: &SdkConfig,
        queue_url: String,
        event_handler: EventHandler,
    ) -> Self {
        Self {
            queue_url,
            event_handler,
            sqs_client: aws_sdk_sqs::Client::new(&aws_configuration),
        }
    }

    pub async fn run(&self, event: LambdaEvent<ApiGatewayProxyRequest>) -> ApiGatewayProxyResponse {
        tracing::info!("Handling API gateway event");

        let (status_code, response_body) = match self.handle_event(event).await {
            Ok(response) => (200, response),
            Err(e) => {
                tracing::error!(
                    "Failed to handle API gateway event: {:#}",
                    anyhow::Error::from(e)
                );
                (500, String::from("Internal Server Error"))
            }
        };

        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        ApiGatewayProxyResponse {
            status_code,
            multi_value_headers: headers.clone(),
            is_base64_encoded: false,
            body: Some(Body::Text(response_body)),
            headers,
        }
    }

    async fn handle_event(
        &self,
        event: LambdaEvent<ApiGatewayProxyRequest>,
    ) -> qr_error::Result<String> {
        let (proxy_request, context) = event.into_parts();

        let proxy_request_body = proxy_request
            .body
            .ok_or(qr_error::Error::MissingRequestBody)?;

        tracing::trace!("Deserializing request '{proxy_request_body}' from JSON");
        let request: EventHandler::Request =
            serde_json::from_str(&proxy_request_body).map_err(|e| {
                qr_error::Error::DeserializeApiGatewayRequest(e, proxy_request_body.clone())
            })?;

        tracing::trace!("Invoking event handler");
        let (queue_message, response) = self.event_handler.handle_event(&context, &request).await?;

        let queue_message_json = serde_json::to_string(&queue_message)
            .map_err(qr_error::Error::SerializeQueueMessage)?;
        tracing::trace!("Serialized SQS message to JSON: '{}'", queue_message_json);

        tracing::info!(
            "Writing message '{}' to SQS '{}",
            queue_message_json,
            self.queue_url
        );
        self.sqs_client
            .send_message()
            .queue_url(&self.queue_url)
            .message_body(&queue_message_json)
            .send()
            .await
            .map_err(|e| {
                qr_error::Error::SendSqsMessage(e, queue_message_json, self.queue_url.clone())
            })?;
        tracing::info!("Successfully written message to SQS");

        let response_json = serde_json::to_string(&response)
            .map_err(qr_error::Error::SerializeApiGatewayResponse)?;
        tracing::trace!("Serialized resonse to JSON: '{}'", response_json);

        Ok(response_json)
    }
}

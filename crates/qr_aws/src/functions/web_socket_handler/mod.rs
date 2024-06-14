mod handle_event;

pub use handle_event::HandleEvent;

use aws_lambda_events::{
    apigw::{ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest},
    encodings::Body,
    http::HeaderMap,
};
use lambda_runtime::{tracing, LambdaEvent};
use serde_json::Value;

pub struct Function<EventHandler> {
    event_handler: EventHandler,
}

const WEB_SOCKET_REQUEST_DATA_KEY: &str = "data";

impl<EventHandler> Function<EventHandler>
where
    EventHandler: HandleEvent,
{
    pub fn new(event_handler: EventHandler) -> Self {
        Self { event_handler }
    }

    pub async fn run(
        &self,
        event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
    ) -> ApiGatewayProxyResponse {
        tracing::info!("Handling WebSocket event");

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
        event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
    ) -> qr_error::Result<String> {
        let (websocket_request, context) = event.into_parts();

        let connection_id = websocket_request
            .request_context
            .connection_id
            .ok_or(qr_error::Error::NoWebSocketConnectionId)?;

        tracing::info!("WebSocket connection id: '{}'", connection_id);

        let request: Option<<EventHandler as HandleEvent>::Request> = websocket_request
            .body
            .as_ref()
            .map(|body| self.deserialize_request(body))
            .transpose()?;

        let response = self
            .event_handler
            .handle_event(&context, &connection_id, request.as_ref())
            .await?;

        let response_json =
            serde_json::to_string(&response).map_err(qr_error::Error::SerializeResponse)?;
        tracing::trace!("Serialized response to JSON: '{}'", response_json);

        Ok(response_json)
    }

    fn deserialize_request(
        &self,
        body: &str,
    ) -> qr_error::Result<<EventHandler as HandleEvent>::Request> {
        tracing::trace!("Deserializing request '{body}' from JSON");
        let deserialized_body: Value = serde_json::from_str(&body)
            .map_err(|e| qr_error::Error::DeserializeRequest(e, body.to_owned()))?;

        let Value::Object(mut deserialized_body_object) = deserialized_body else {
            return Err(qr_error::Error::WebSocketRequestNotAnObject(
                body.to_owned(),
            ));
        };

        let data = deserialized_body_object
            .remove(WEB_SOCKET_REQUEST_DATA_KEY)
            .ok_or_else(|| qr_error::Error::WebSocketRequestMissingData(body.to_owned()))?;

        serde_json::from_value(data)
            .map_err(|e| qr_error::Error::DeserializeRequest(e, body.to_owned()))
    }
}

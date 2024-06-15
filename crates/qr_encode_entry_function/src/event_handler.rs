use lambda_runtime::{tracing, Context};
use qr_aws::functions::api_gateway_to_sqs::HandleEvent;
use qr_encode_result_function::QrEncodeResultInput;

use crate::{qr_encode_request::QrEncodeRequest, qr_encode_response::QrEncodeResponse};

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl HandleEvent for EventHandler {
    type Request = QrEncodeRequest;
    type Response = QrEncodeResponse;
    type QueueMessage = QrEncodeResultInput;

    async fn handle_event(
        &self,
        context: &Context,
        request: &Self::Request,
    ) -> qr_error::Result<(Self::QueueMessage, Self::Response)> {
        tracing::info!(
            "Handling encode entry event: request_id='{}'",
            context.request_id
        );

        let queue_message =
            QrEncodeResultInput::new(context.request_id.clone(), request.get_data().to_owned());

        let response = QrEncodeResponse::new(context.request_id.clone());

        Ok((queue_message, response))
    }
}

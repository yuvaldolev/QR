use lambda_runtime::Context;
use qr_aws::ProcessEvent;
use qr_encode_result_function::QrEncodeResultRequest;

use crate::{qr_encode_request::QrEncodeRequest, qr_encode_response::QrEncodeResponse};

pub struct EncodeEntryEventProcessor;

impl EncodeEntryEventProcessor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ProcessEvent for EncodeEntryEventProcessor {
    type Request = QrEncodeRequest;
    type Response = QrEncodeResponse;
    type QueueMessage = QrEncodeResultRequest;

    async fn process_event(
        &self,
        request: &Self::Request,
        context: &Context,
    ) -> qr_error::Result<(Self::QueueMessage, Self::Response)> {
        let queue_message = QrEncodeResultRequest::new(
            context.request_id.clone(),
            request.get_data().to_owned(),
            request.get_error_correction_level(),
        );

        let response = QrEncodeResponse::new(context.request_id.clone());

        Ok((queue_message, response))
    }
}

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
        request: Self::Request,
    ) -> qr_error::Result<(Self::QueueMessage, Self::Response)> {
        let queue_message = QrEncodeResultRequest::new(
            request.get_data().to_owned(),
            request.get_error_correction_level(),
        );

        let response = QrEncodeResponse::new(queue_message.get_id());

        Ok((queue_message, response))
    }
}

use aws_config::SdkConfig;
use lambda_runtime::tracing;
use qr_encoder::Request;

use crate::{QrEncodeRequest, QrEncodeResponse};

pub struct QrEncodeEntryFunction {
    request: QrEncodeRequest,
    sqs_url: String,
    sqs_client: aws_sdk_sqs::Client,
}

impl QrEncodeEntryFunction {
    pub fn new(request: QrEncodeRequest, aws_configuration: SdkConfig, sqs_url: String) -> Self {
        Self {
            request,
            sqs_url,
            sqs_client: aws_sdk_sqs::Client::new(&aws_configuration),
        }
    }

    pub async fn run(&self) -> qr_error::Result<QrEncodeResponse> {
        let request = Request::new(
            self.request.get_data().to_owned(),
            self.request.get_error_correction_level(),
        );

        tracing::trace!("Serializing request '{request}' to JSON");
        let serialized_request = serde_json::to_string(&request)
            .map_err(|e| qr_error::Error::SerializeRequest(e, request.to_string()))?;

        tracing::info!("Writing request '{}' to SQS '{}", request, self.sqs_url);
        self.sqs_client
            .send_message()
            .queue_url(&self.sqs_url)
            .message_body(&serialized_request)
            .send()
            .await
            .map_err(|e| {
                qr_error::Error::SendSqsMessage(e, serialized_request, self.sqs_url.clone())
            })?;

        Ok(QrEncodeResponse::new(request.get_id()))
    }
}

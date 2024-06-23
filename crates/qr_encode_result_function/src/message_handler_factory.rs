use aws_config::SdkConfig;
use qr_aws::functions::sqs_handler::MakeMessageHandler;

use crate::message_handler::MessageHandler;

pub struct MessageHandlerFactory {
    table_name: String,
    dynamodb_client: aws_sdk_dynamodb::Client,
    source_queue_url: String,
    sqs_client: aws_sdk_sqs::Client,
    api_gateway_management_client: aws_sdk_apigatewaymanagement::Client,
}

impl MessageHandlerFactory {
    pub fn new(
        aws_configuration: &SdkConfig,
        api_gateway_management_aws_configuration: &SdkConfig,
        table_name: String,
        source_queue_url: String,
    ) -> Self {
        Self {
            table_name,
            dynamodb_client: aws_sdk_dynamodb::Client::new(aws_configuration),
            source_queue_url,
            sqs_client: aws_sdk_sqs::Client::new(aws_configuration),
            api_gateway_management_client: aws_sdk_apigatewaymanagement::Client::new(
                api_gateway_management_aws_configuration,
            ),
        }
    }
}

impl MakeMessageHandler for MessageHandlerFactory {
    type MessageHandler = MessageHandler;

    fn make_message_handler(&self) -> Self::MessageHandler {
        MessageHandler::new(
            self.table_name.clone(),
            self.dynamodb_client.clone(),
            self.source_queue_url.clone(),
            self.sqs_client.clone(),
            self.api_gateway_management_client.clone(),
        )
    }
}

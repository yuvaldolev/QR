use aws_config::SdkConfig;
use qr_aws::functions::sqs_handler::MakeMessageHandler;

use crate::message_handler::MessageHandler;

pub struct MessageHandlerFactory {
    table_name: String,
    dynamodb_client: aws_sdk_dynamodb::Client,
    api_gateway_management_client: aws_sdk_apigatewaymanagement::Client,
}

impl MessageHandlerFactory {
    pub fn new(
        dynamodb_aws_configuration: &SdkConfig,
        api_gateway_management_aws_configuration: &SdkConfig,
        table_name: String,
    ) -> Self {
        Self {
            table_name,
            dynamodb_client: aws_sdk_dynamodb::Client::new(dynamodb_aws_configuration),
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
            self.api_gateway_management_client.clone(),
        )
    }
}

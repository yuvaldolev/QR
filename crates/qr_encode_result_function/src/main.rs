use std::env;

use aws_lambda_events::sqs::{SqsBatchResponse, SqsEventObj};
use lambda_runtime::{tracing, Error, LambdaEvent};
use qr_aws::functions::sqs_handler::Function;
use qr_encode_result_function::MessageHandlerFactory;
use serde_json::Value;

async fn function_handler(
    event: LambdaEvent<SqsEventObj<Value>>,
) -> Result<SqsBatchResponse, Error> {
    let table_name =
        env::var("TABLE_NAME").expect("environment variable `TABLE_NAME` should be set");
    let source_queue_url = env::var("SOURCE_QUEUE_URL")
        .expect("environment variable `SOURCE_QUEUE_URL` should be set");
    let web_socket_api_endpoint = env::var("WEB_SOCKET_API_ENDPOINT")
        .expect("environment variable `WEB_SOCKET_API_ENDPOINT` should be set");

    let aws_configuration = aws_config::load_from_env().await;
    let api_gateway_management_aws_configuration = aws_config::from_env()
        .endpoint_url(web_socket_api_endpoint)
        .load()
        .await;

    let function = Function::new(MessageHandlerFactory::new(
        &aws_configuration,
        &api_gateway_management_aws_configuration,
        table_name,
        source_queue_url,
    ));
    let response = function.run(event).await;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let function = lambda_runtime::service_fn(function_handler);
    lambda_runtime::run(function).await?;

    Ok(())
}

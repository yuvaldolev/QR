use std::env;

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_runtime::{tracing, Error, LambdaEvent};
use qr_aws::functions::api_gateway_to_sqs::Function;
use qr_encode_entry_function::EventHandler;

async fn function_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let aws_configuration = aws_config::load_from_env().await;
    let queue_url = env::var("QUEUE_URL").expect("environment variable `QUEUE_URL` should be set");

    let function = Function::new(&aws_configuration, queue_url, EventHandler::new());
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

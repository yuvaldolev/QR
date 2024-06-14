use std::env;

use aws_lambda_events::apigw::{ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest};
use lambda_runtime::{tracing, Error, LambdaEvent};
use qr_aws::functions::web_socket_handler::Function;
use qr_result_web_socket_disconnect_function::EventHandler;

async fn function_handler(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let aws_configuration = aws_config::load_from_env().await;
    let table_name =
        env::var("TABLE_NAME").expect("environment variable `TABLE_NAME` should be set");

    let function = Function::new(EventHandler::new(&aws_configuration, table_name));
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

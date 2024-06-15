use aws_lambda_events::apigw::{ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest};
use lambda_runtime::{tracing, Error, LambdaEvent};
use qr_aws::functions::web_socket_handler::Function;
use qr_result_web_socket_connect_function::EventHandler;

async fn function_handler(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let function = Function::new(EventHandler::new());
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

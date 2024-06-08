use aws_lambda_events::{
    apigw::{ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest},
    encodings::Body,
    http::HeaderMap,
};
use lambda_runtime::{tracing, Error, LambdaEvent};

async fn function_handler(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let (websocket_request, _) = event.into_parts();

    let connection_id = websocket_request.request_context.connection_id.unwrap();

    tracing::info!(
        "WebSocket connection request: connection='{}'",
        connection_id
    );

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/plain".parse().unwrap());

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        multi_value_headers: headers.clone(),
        is_base64_encoded: false,
        body: Some(Body::Text(String::from("Connected!"))),
        headers,
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let function = lambda_runtime::service_fn(function_handler);
    lambda_runtime::run(function).await?;

    Ok(())
}

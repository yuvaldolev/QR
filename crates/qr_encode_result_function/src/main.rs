use std::env;

use aws_lambda_events::{
    apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse},
    encodings::Body,
    http::HeaderMap,
};
use lambda_runtime::{tracing, Error, LambdaEvent};
use qr_encode_entry_function::{QrEncodeEntryFunction, QrEncodeRequest};

async fn try_handle(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<String, Error> {
    let (proxy_request, _) = event.into_parts();

    let proxy_request_body = proxy_request.body.unwrap();

    let request: QrEncodeRequest = serde_json::from_str(&proxy_request_body)?;
    let aws_configuration = aws_config::load_from_env().await;
    let sqs_url = env::var("SQS_URL")?;

    let function = QrEncodeEntryFunction::new(request, aws_configuration, sqs_url);
    let response = function.run().await?;

    let response_json = serde_json::to_string(&response)?;

    Ok(response_json)
}

async fn function_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let (status_code, body) = match try_handle(event).await {
        Ok(response) => (200, response),
        Err(e) => {
            tracing::error!("Failed to handle request: {e}");
            (500, String::from("Internal Server Error"))
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    Ok(ApiGatewayProxyResponse {
        status_code,
        multi_value_headers: headers.clone(),
        is_base64_encoded: false,
        body: Some(Body::Text(body)),
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

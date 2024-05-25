use std::env;

use aws_lambda_events::{apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse}, encodings::Body, http::HeaderMap};
use lambda_runtime::{tracing, Context, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    data: String,
    // error_correction_level: ErrorCorrectionLevel,
}

#[derive(Serialize)]
struct Response {
    id: String,
}

async fn run(context: Context, request: Request) -> Result<Response, Error> {
    let aws_configuration = aws_config::load_from_env().await;
    let sqs_client = aws_sdk_sqs::Client::new(&aws_configuration);

    let sqs_url = env::var("SQS_URL").expect("SQS_URL environment variable is not set");

    tracing::info!("Writing request '{request:?}' to SQS '{sqs_url}");

    let sqs_response = match sqs_client.send_message()
        .queue_url(sqs_url)
        .message_body(serde_json::to_string(&request)?)
        .send()
        .await {
            Ok(sqs_response) => sqs_response,
            Err(e) => {
                tracing::error!("Failed to write to SQS: {e}");
                return Err(e.into());
            }
        };

    tracing::info!("SQS response: {sqs_response:?}");

    Ok(Response {
        id: context.request_id,
    })
}

async fn try_handle(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<String, Error> {
    let (proxy_request, context) = event.into_parts();

    let proxy_request_body = proxy_request.body.unwrap();
    let request: Request = serde_json::from_str(&proxy_request_body)?;

    let response = run(context, request).await?;
    let response_json = serde_json::to_string(&response)?;

    Ok(response_json)
}

async fn function_handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
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

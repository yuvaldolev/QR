use lambda_runtime::{Error, LambdaEvent, tracing};

use qr_encoder::ErrorCorrectionLevel;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    data: String,
    error_correction_level: ErrorCorrectionLevel,
}

#[derive(Serialize)]
struct Response {
    statusCode: i32,
    body: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Prepare the response
    let resp = Response {
        statusCode: 200,
        body: "Updated Hello World from Rust!".to_string(),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let function = lambda_runtime::service_fn(function_handler);
    lambda_runtime::run(function).await
}

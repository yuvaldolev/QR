use clap::Parser;
use futures_util::StreamExt;
use qr_cli::Cli;
use qr_encode_entry_function::{QrEncodeRequest, QrEncodeResponse};
use reqwest::Client;

const ENCODE_API_URL: &str = env!("ENCODE_API_URL");
// const ENCODE_RESULT_WEBSOCKET_URL: &str = env!("ENCODE_RESULT_WEBSOCKET_URL");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let encode_request = QrEncodeRequest::new(cli.data, cli.error_correction_level.into());
    println!("Sending request to: {}", ENCODE_API_URL);
    let client = Client::new();
    let response: QrEncodeResponse = client
        .post(ENCODE_API_URL)
        .json(&encode_request)
        .send()
        .await?
        .json()
        .await?;
    println!("Response: {:?}", response);

    // let (websocket, _) = tokio_tungstenite::connect_async(ENCODE_RESULT_WEBSOCKET_URL).await?;
    // websocket.split();

    Ok(())
}

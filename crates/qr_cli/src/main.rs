use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use qr_cli::Cli;
use qr_encode_entry_function::{QrEncodeRequest, QrEncodeResponse};
use qr_encode_result_function::QrEncodeResultOutput;
use qr_result_web_socket_monitor_function::QrResultMonitorRequest;
use reqwest::Client;
use tokio_tungstenite::tungstenite::Message;

const ENCODE_API_URL: &str = env!("ENCODE_API_URL");
const RESULT_WEB_SOCKET_API_URL: &str = env!("RESULT_WEB_SOCKET_API_URL");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let encode_request = QrEncodeRequest::new(cli.data, cli.error_correction_level.into());
    println!("Sending encode request to: {}", ENCODE_API_URL);
    let client = Client::new();
    let response: QrEncodeResponse = client
        .post(ENCODE_API_URL)
        .json(&encode_request)
        .send()
        .await?
        .json()
        .await?;
    println!(
        "Received encode response: request_id={}",
        response.get_request_id()
    );

    println!(
        "Connecting to result websocket: {}",
        RESULT_WEB_SOCKET_API_URL
    );
    let (mut web_socket, _) = tokio_tungstenite::connect_async(RESULT_WEB_SOCKET_API_URL).await?;
    println!("Successfully connected to result websocket");

    println!("Sending monitor request to result websocket");
    let result_monitor_request = QrResultMonitorRequest::new(response.get_request_id().to_owned());
    let result_monitor_web_socket_message = serde_json::json!({
        "action": "monitor",
        "data": result_monitor_request,
    });
    web_socket
        .send(Message::Text(serde_json::to_string(
            &result_monitor_web_socket_message,
        )?))
        .await?;

    println!("Waiting for result from websocket");
    let result_web_socket_message = web_socket
        .next()
        .await
        .ok_or(anyhow::anyhow!("No message received from websocket"))??;
    let result: QrEncodeResultOutput = serde_json::from_str(result_web_socket_message.to_text()?)?;

    println!("Received result: {}", result.get_data());

    Ok(())
}

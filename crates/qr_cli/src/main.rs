use reqwest::blocking::Client;

mod cli;

const QR_ENCODE_API_URL: &str = env!("QR_ENCODE_API_URL");

fn main() -> anyhow::Result<()> {
    // let cli = Cli::parse();

    // let encoder = Encoder::new(cli.error_correction_level.into_error_correction_level());
    // encoder.encode(&cli.data)?;

    let client = Client::new();

    println!("Sending request to {}", QR_ENCODE_API_URL);
    let response = client.post(QR_ENCODE_API_URL).send()?;
    println!("Response: {:?}", response);

    Ok(())
}

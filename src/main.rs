use clap::Parser;
use qr_encoder::Encoder;

#[derive(Parser)]
struct Cli {
    data: String,
}

fn main() {
    let cli = Cli::parse();

    let encoder = Encoder::new();
    encoder.encode(&cli.data);
}

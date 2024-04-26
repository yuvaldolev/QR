use clap::Parser;
use cli::Cli;
use qr_encoder::Encoder;

mod cli;

fn main() {
    let cli = Cli::parse();

    let encoder = Encoder::new(cli.error_correction_level.into_error_correction_level());
    encoder.encode(&cli.data);
}

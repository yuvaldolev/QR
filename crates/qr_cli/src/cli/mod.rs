mod error_correction_level_cli;

use clap::Parser;
use error_correction_level_cli::ErrorCorrectionLevelCli;

#[derive(Parser)]
pub struct Cli {
    pub data: String,

    #[arg(short, long, default_value_t = ErrorCorrectionLevelCli::M)]
    pub error_correction_level: ErrorCorrectionLevelCli,
}

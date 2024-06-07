use std::fmt::Display;

use clap::ValueEnum;
use qr_encoder::ErrorCorrectionLevel;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum ErrorCorrectionLevelCli {
    L,
    M,
    Q,
    H,
}

impl Display for ErrorCorrectionLevelCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::L => write!(f, "l"),
            Self::M => write!(f, "m"),
            Self::Q => write!(f, "q"),
            Self::H => write!(f, "h"),
        }
    }
}

impl From<ErrorCorrectionLevelCli> for ErrorCorrectionLevel {
    fn from(error_correction_level_cli: ErrorCorrectionLevelCli) -> Self {
        match error_correction_level_cli {
            ErrorCorrectionLevelCli::L => ErrorCorrectionLevel::Low,
            ErrorCorrectionLevelCli::M => ErrorCorrectionLevel::Medium,
            ErrorCorrectionLevelCli::Q => ErrorCorrectionLevel::Quartile,
            ErrorCorrectionLevelCli::H => ErrorCorrectionLevel::High,
        }
    }
}

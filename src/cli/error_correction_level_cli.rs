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

impl ErrorCorrectionLevelCli {
    pub fn into_error_correction_level(self) -> ErrorCorrectionLevel {
        match self {
            Self::L => ErrorCorrectionLevel::Low,
            Self::M => ErrorCorrectionLevel::Medium,
            Self::Q => ErrorCorrectionLevel::Quartile,
            Self::H => ErrorCorrectionLevel::High,
        }
    }
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

use qr_encoder::ErrorCorrectionLevel;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QrEncodeRequest {
    data: String,
    error_correction_level: ErrorCorrectionLevel,
}

impl QrEncodeRequest {
    pub fn new(data: String, error_correction_level: ErrorCorrectionLevel) -> Self {
        Self {
            data,
            error_correction_level,
        }
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }

    pub fn get_error_correction_level(&self) -> ErrorCorrectionLevel {
        self.error_correction_level
    }
}

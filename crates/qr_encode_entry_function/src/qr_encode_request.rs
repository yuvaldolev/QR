use qr_encoder::ErrorCorrectionLevel;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QrEncodeRequest {
    data: String,
    error_correction_level: ErrorCorrectionLevel,
}

impl QrEncodeRequest {
    pub fn get_data(&self) -> &str {
        &self.data
    }

    pub fn get_error_correction_level(&self) -> ErrorCorrectionLevel {
        self.error_correction_level
    }
}

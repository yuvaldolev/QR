use qr_encoder::ErrorCorrectionLevel;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QrEncodeResultRequest {
    id: String,
    data: String,
    error_correction_level: ErrorCorrectionLevel,
}

impl QrEncodeResultRequest {
    pub fn new(id: String, data: String, error_correction_level: ErrorCorrectionLevel) -> Self {
        Self {
            id,
            data,
            error_correction_level,
        }
    }
}

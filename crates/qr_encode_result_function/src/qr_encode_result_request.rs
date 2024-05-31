use std::fmt::{self, Display, Formatter};

use qr_encoder::ErrorCorrectionLevel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct QrEncodeResultRequest {
    id: u128,
    data: String,
    error_correction_level: ErrorCorrectionLevel,
}

impl QrEncodeResultRequest {
    pub fn new(data: String, error_correction_level: ErrorCorrectionLevel) -> Self {
        Self {
            id: Uuid::new_v4().as_u128(),
            data,
            error_correction_level,
        }
    }

    pub fn get_id(&self) -> u128 {
        self.id
    }
}

impl Display for QrEncodeResultRequest {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "QrEncodeResultRequest {{ id: {}, data: {}, error_correction_level: {} }}",
            self.id, self.data, self.error_correction_level
        )
    }
}

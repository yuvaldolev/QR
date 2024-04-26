use crate::ErrorCorrectionLevel;

use super::data_encoder::DataEncoder;

pub struct NumericDataEncoder {
    error_correction_level: ErrorCorrectionLevel,
}

impl NumericDataEncoder {
    pub fn new(error_correction_level: ErrorCorrectionLevel) -> Self {
        Self {
            error_correction_level,
        }
    }
}

impl DataEncoder for NumericDataEncoder {
    fn encode(&self, data: &str) -> Vec<u8> {
        Vec::new()
    }
}

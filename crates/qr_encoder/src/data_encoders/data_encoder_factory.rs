use crate::{data_encoding::DataEncoding, ErrorCorrectionLevel};

use super::{data_encoder::DataEncoder, numeric_data_encoder::NumericDataEncoder};

pub struct DataEncoderFactory {
    error_correction_level: ErrorCorrectionLevel,
}

impl DataEncoderFactory {
    pub fn new(error_correction_level: ErrorCorrectionLevel) -> Self {
        Self {
            error_correction_level,
        }
    }

    pub fn make(&self, encoding: &DataEncoding) -> Box<dyn DataEncoder> {
        match encoding {
            DataEncoding::Numeric => {
                Box::new(NumericDataEncoder::new(self.error_correction_level.clone()))
            }
            DataEncoding::Alphanumeric => todo!(),
            DataEncoding::Byte => todo!(),
        }
    }
}

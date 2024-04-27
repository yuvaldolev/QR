use crate::data_encoding::DataEncoding;

use super::{data_encoder::DataEncoder, numeric_data_encoder::NumericDataEncoder};

pub struct DataEncoderFactory;

impl DataEncoderFactory {
    pub fn new() -> Self {
        Self
    }

    pub fn make(&self, encoding: &DataEncoding) -> Box<dyn DataEncoder> {
        match encoding {
            DataEncoding::Numeric => Box::new(NumericDataEncoder::new()),
            DataEncoding::Alphanumeric => todo!(),
            DataEncoding::Byte => todo!(),
        }
    }
}

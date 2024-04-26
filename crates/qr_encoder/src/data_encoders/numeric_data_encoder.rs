use super::data_encoder::DataEncoder;

pub struct NumericDataEncoder;

impl NumericDataEncoder {
    pub fn new() -> Self {
        Self
    }
}

impl DataEncoder for NumericDataEncoder {
    fn encode(&self, data: &str) -> Vec<u8> {
        Vec::new()
    }
}

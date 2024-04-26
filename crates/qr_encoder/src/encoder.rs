use crate::{data_analyzer::DataAnalyzer, data_encoders::DataEncoderFactory};

pub struct Encoder;

impl Encoder {
    pub fn new() -> Self {
        Self
    }

    pub fn encode(&self, data: &str) {
        let data_analyzer = DataAnalyzer::new();
        let data_encoding = data_analyzer.analyze(data);

        let data_encoder_factory = DataEncoderFactory::new();
        let data_encoder = data_encoder_factory.make(&data_encoding);
        let encoded_data = data_encoder.encode(data);
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

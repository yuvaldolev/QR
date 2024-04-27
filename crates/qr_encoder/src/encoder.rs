use crate::{data_analyzer::DataAnalyzer, data_encoders::DataEncoderFactory, ErrorCorrectionLevel};

pub struct Encoder {
    error_correction_level: ErrorCorrectionLevel,
}

impl Encoder {
    pub fn new(error_correction_level: ErrorCorrectionLevel) -> Self {
        Self {
            error_correction_level,
        }
    }

    pub fn encode(&self, data: &str) {
        let data_analyzer = DataAnalyzer::new();
        let data_encoding = data_analyzer.analyze(data);

        let data_encoder_factory = DataEncoderFactory::new();
        let data_encoder = data_encoder_factory.make(&data_encoding);
        let encoded_data = data_encoder.encode(data);
        println!("Encoded: {}", encoded_data);
    }
}

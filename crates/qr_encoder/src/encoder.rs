use crate::data_analyzer::DataAnalyzer;

pub struct Encoder;

impl Encoder {
    pub fn new() -> Self {
        Self
    }

    pub fn encode(&self, data: &str) {
        let data_analyzer = DataAnalyzer::new();
        data_analyzer.analyze(data);
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

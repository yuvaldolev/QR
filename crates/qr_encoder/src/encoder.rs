use crate::{
    data_analyzer::DataAnalyzer, error, segment_encoder::SegmentEncoder,
    version_analyzer::VersionAnalyzer, ErrorCorrectionLevel,
};

pub struct Encoder {
    error_correction_level: ErrorCorrectionLevel,
    data_analyzer: DataAnalyzer,
    version_analyzer: VersionAnalyzer,
    segment_encoder: SegmentEncoder,
}

impl Encoder {
    pub fn new(error_correction_level: ErrorCorrectionLevel) -> Self {
        Self {
            error_correction_level,
            data_analyzer: DataAnalyzer::new(),
            version_analyzer: VersionAnalyzer::new(),
            segment_encoder: SegmentEncoder::new(),
        }
    }

    pub fn encode(&self, data: &str) -> error::Result<()> {
        let data_encoding = self.data_analyzer.analyze(data);
        let version = self.version_analyzer.analyze(
            data.len(),
            &data_encoding,
            &self.error_correction_level,
        )?;
        println!("version: {}", version);
        // let segment = self.segment_encoder.encode(data, &data_encoding);

        Ok(())
    }
}

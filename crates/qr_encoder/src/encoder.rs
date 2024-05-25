use bitvec::{order::Msb0, vec::BitVec};

use crate::{
    data_analyzer::DataAnalyzer, segment::Segment, segment_encoder::SegmentEncoder,
    version::Version, version_analyzer::VersionAnalyzer, ErrorCorrectionLevel,
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

    pub fn encode(&self, data: &str) -> qr_error::Result<()> {
        let segments = self.data_analyzer.analyze(data);
        let version = self
            .version_analyzer
            .analyze(&segments, self.error_correction_level)?;

        let encoded_segments = self.encode_segments(&segments, data, &version);

        Ok(())
    }

    fn encode_segments(
        &self,
        segments: &[Segment],
        data: &str,
        version: &Version,
    ) -> BitVec<u8, Msb0> {
        let mut encoded_segments: BitVec<u8, Msb0> = BitVec::new();

        for segment in segments {
            let mut encoded_segment = self.segment_encoder.encode(segment, data, version);
            encoded_segments.append(&mut encoded_segment);
        }

        encoded_segments
    }
}

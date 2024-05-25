use crate::{segment::Segment, version::Version, ErrorCorrectionLevel};

pub struct VersionAnalyzer;

impl VersionAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn analyze(
        &self,
        segments: &[Segment],
        error_correction_level: ErrorCorrectionLevel,
    ) -> qr_error::Result<Version> {
        let mut low_version = Version::new(1);

        for version in &[Version::new(9), Version::new(26), Version::new(40)] {
            let total_encoded_length = Self::calculate_total_encoded_length(segments, version);
            let version_data_capacity = version.get_data_capacity(error_correction_level);
            if total_encoded_length > version_data_capacity {
                low_version = Version::new(version.get_number() + 1);
                continue;
            }

            return Ok(Self::find_minimum_version(
                total_encoded_length,
                &low_version,
                version,
                error_correction_level,
            ));
        }

        Err(qr_error::Error::DataTooLong)
    }

    fn calculate_total_encoded_length(segments: &[Segment], version: &Version) -> usize {
        segments
            .iter()
            .map(|segment| segment.encoded_length(version))
            .sum()
    }

    fn find_minimum_version(
        required_length: usize,
        low_version: &Version,
        high_version: &Version,
        error_correction_level: ErrorCorrectionLevel,
    ) -> Version {
        let mut minimum_version = Version::new(low_version.get_number());
        for version in (low_version.get_number()..=high_version.get_number()).map(Version::new) {
            let data_capacity = version.get_data_capacity(error_correction_level);
            if required_length <= data_capacity {
                minimum_version = version;
                break;
            }
        }

        minimum_version
    }
}

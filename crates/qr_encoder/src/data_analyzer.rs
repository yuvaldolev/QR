use crate::{
    alphanumeric_encoding_table::AlphanumericEncodingTable, data_encoding::DataEncoding,
    segment::Segment,
};

pub struct DataAnalyzer {
    alphanumeric_encoding_table: AlphanumericEncodingTable,
}

impl DataAnalyzer {
    pub fn new() -> Self {
        Self {
            alphanumeric_encoding_table: AlphanumericEncodingTable::new(),
        }
    }

    pub fn analyze(&self, data: &str) -> Vec<Segment> {
        // TODO: Optimize data segmentation and return multiple segments with most
        // efficient encoding per-segment.
        //
        // TODO: Kanji encoding support.

        let encoding = if Self::is_numeric_encodable(data) {
            DataEncoding::Numeric
        } else if self.is_alphanumeric_encodable(data) {
            DataEncoding::Alphanumeric
        } else {
            DataEncoding::Byte
        };

        vec![Segment::new(0, data.len(), encoding)]
    }

    fn is_numeric_encodable(data: &str) -> bool {
        data.chars().all(|c| c.is_ascii_digit())
    }

    fn is_alphanumeric_encodable(&self, data: &str) -> bool {
        data.chars()
            .all(|c| self.alphanumeric_encoding_table.contains(c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_analyze {
        ($($name:ident: $data:literal -> $expected_data_encoding:expr),+) => {
            $(
                #[test]
                fn $name() {
                    let data_analyzer = DataAnalyzer::new();
                    let segments = data_analyzer.analyze($data);

                    assert_eq!(segments.len(), 1);

                    let segment = &segments[0];
                    assert_eq!(segment.get_start(), 0);
                    assert_eq!(segment.get_end(), $data.len());
                    assert_eq!(segment.get_encoding(), &$expected_data_encoding);
                }
            )+
        };
    }

    test_analyze! {
        test_analyze_numeric: "1234567890" -> DataEncoding::Numeric,
        test_analyze_alphanumeric_uppercase_characters:
            "ABCD" -> DataEncoding::Alphanumeric,
        test_analyze_alphanumeric_uppercase_characters_and_digits:
            "CHARACTER123AND456DIGIT" -> DataEncoding::Alphanumeric,
        test_analyze_alphanumeric_uppercase_characters_digits_and_symbols:
            "C H$A%R*A+C-T.E/RS1:23" -> DataEncoding::Alphanumeric,
        test_analyze_byte_lowercase_characters: "abcd" -> DataEncoding::Byte,
        test_analyze_byte_lowercase_and_uppercase_characters: "abCD" -> DataEncoding::Byte,
        test_analyze_byte_lowercase_characters_and_digits:
            "12ab12" -> DataEncoding::Byte,
        test_analyze_byte_lowercase_and_uppercase_characters_and_digits:
            "12ab34CD56" -> DataEncoding::Byte,
        test_analyze_byte_unicode: "שלום עולם 😀" -> DataEncoding::Byte,
        test_analyze_byte_uppercase_characters_digits_and_symbols_not_in_alphanumeric_table:
            "C H$A%R*A+C-T.E/RS1:23!" -> DataEncoding::Byte
    }
}

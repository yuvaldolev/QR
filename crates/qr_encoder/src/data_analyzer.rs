use crate::data_encoding::DataEncoding;

pub struct DataAnalyzer;

impl DataAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, data: &str) -> DataEncoding {
        if Self::is_numeric_encodable(data) {
            DataEncoding::Numeric
        } else if Self::is_alphanumeric_encodable(data) {
            DataEncoding::Alphanumeric
        } else {
            DataEncoding::Byte
        }
    }

    fn is_numeric_encodable(data: &str) -> bool {
        data.chars().all(|c| c.is_ascii_digit())
    }

    fn is_alphanumeric_encodable(data: &str) -> bool {
        data.chars()
            .all(|c| c.is_ascii_digit() || c.is_ascii_uppercase())
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
                    let data_encoding = data_analyzer.analyze($data);
                    assert_eq!(data_encoding, $expected_data_encoding);
                }
            )+
        };
    }

    test_analyze! {
        test_analyze_numeric: "1234567890" -> DataEncoding::Numeric,
        test_analyze_alphanumeric_uppercase_characters:
            "ABCD" -> DataEncoding::Alphanumeric,
        test_analyze_alphanumeric_uppercase_characters_and_digits:
            "CHARACTERS123AND456DIGITS" -> DataEncoding::Alphanumeric,
        test_analyze_byte_lowercase_characters: "abcd" -> DataEncoding::Byte,
        test_analyze_byte_lowercase_and_uppercase_characters: "abCD" -> DataEncoding::Byte,
        test_analyze_byte_lowercase_characters_and_digits:
            "12ab12" -> DataEncoding::Byte,
        test_analyze_byte_lowercase_and_uppercase_characters_and_digits:
            "12ab34CD56" -> DataEncoding::Byte,
        test_analyze_byte_unicode: "שלום עולם 😀" -> DataEncoding::Byte
    }
}

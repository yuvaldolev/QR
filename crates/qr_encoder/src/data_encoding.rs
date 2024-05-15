#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DataEncoding {
    Numeric,
    Alphanumeric,
    Byte,
}

impl DataEncoding {
    pub fn as_mode_indicator(&self) -> u8 {
        match self {
            DataEncoding::Numeric => 0b0001,
            DataEncoding::Alphanumeric => 0b0010,
            DataEncoding::Byte => 0b0100,
        }
    }

    pub fn get_data_bit_size(&self, character_count: usize) -> usize {
        match self {
            DataEncoding::Numeric => ((character_count * 10) + 2) / 3,
            DataEncoding::Alphanumeric => ((character_count * 11) + 1) / 2,
            DataEncoding::Byte => character_count * 8,
        }
    }
}

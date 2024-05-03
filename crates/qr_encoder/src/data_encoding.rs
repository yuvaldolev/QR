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
}

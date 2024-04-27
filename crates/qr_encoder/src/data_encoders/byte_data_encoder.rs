use bitvec::{order::Msb0, vec::BitVec, view::AsBits};

use super::data_encoder::DataEncoder;

pub struct ByteDataEncoder;

impl ByteDataEncoder {
    pub fn new() -> Self {
        Self
    }
}

impl DataEncoder for ByteDataEncoder {
    fn encode(&self, data: &str) -> BitVec<u8, Msb0> {
        BitVec::from_bitslice(data.as_bits())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_encode {
        ($($name:ident: $data:literal -> $expected_encoded_data:expr),+) => {
            $(
                #[test]
                fn $name() {
                    let encoder = ByteDataEncoder::new();
                    let encoded_data = encoder.encode($data);
                    assert_eq!(encoded_data, $expected_encoded_data);
                }
            )+
        };
    }

    test_encode! {
        test_encode_lowercase_characters: "abcd" -> bitvec::bitvec![
            u8, Msb0;
            0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0,
            1, 1, 0, 0, 1, 0, 0,
        ],
        test_encode_lowercase_and_uppercase_characters: "abCD" -> bitvec::bitvec![
            u8, Msb0;
            0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0,
            1, 0, 0, 0, 1, 0, 0,
        ],
        test_encode_lowercase_characters_and_digits: "12ab12" -> bitvec::bitvec![
            u8, Msb0;
            0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0,
            1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0,
        ],
        test_encode_lowercase_and_uppercase_characters_and_digits: "12ab34CD56" -> bitvec::bitvec![
            u8, Msb0;
            0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0,
            1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1,
            0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1,
            1, 0, 1, 1, 0,
        ],
        test_encode_unicode: "שלום עולם 😀" -> bitvec::bitvec![
            u8, Msb0;
            1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1,
            0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1,
            0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0,
            1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1,
            0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0,
            1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0,
            0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            0,
        ],
        test_encode_uppercase_characters_digits_and_symbols_not_in_alphanumeric_table: "C H$A%R*A+C-T.E/RS1:23!" -> bitvec::bitvec![
            u8, Msb0;
            0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0,
            0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1,
            0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1,
            0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1,
            0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1,
            1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0,
            0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1,
            1, 0, 0, 1, 0, 0, 0, 0, 1,
        ]
    }
}

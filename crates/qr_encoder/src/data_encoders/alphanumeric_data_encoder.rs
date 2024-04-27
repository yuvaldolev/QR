use bitvec::{field::BitField, order::Msb0, vec::BitVec};
use itertools::Itertools;

use crate::alphanumeric_encoding_table::AlphanumericEncodingTable;

use super::data_encoder::DataEncoder;

const CHUNK_SIZE: usize = 2;
const EVEN_CHUNK_BIT_SIZE: usize = 11;
const ODD_CHUNK_BIT_SIZE: usize = 6;
const FIRST_NUMBER_MULTIPLIER: u16 = 45;

pub struct AlphanumericDataEncoder {
    alphanumeric_encoding_table: AlphanumericEncodingTable,
}

impl AlphanumericDataEncoder {
    pub fn new() -> Self {
        Self {
            alphanumeric_encoding_table: AlphanumericEncodingTable::new(),
        }
    }

    fn encode_chunk(&self, chunk: &str) -> BitVec<u8, Msb0> {
        if CHUNK_SIZE == chunk.len() {
            self.encode_even_chunk(chunk)
        } else {
            self.encode_odd_chunk(chunk)
        }
    }

    fn encode_even_chunk(&self, chunk: &str) -> BitVec<u8, Msb0> {
        let encoded_first_character = self
            .alphanumeric_encoding_table
            .get(chunk.chars().nth(0).unwrap())
            .unwrap();
        let encoded_second_character = self
            .alphanumeric_encoding_table
            .get(chunk.chars().nth(1).unwrap())
            .unwrap();

        let encoded_number = ((encoded_first_character as u16) * FIRST_NUMBER_MULTIPLIER)
            + (encoded_second_character as u16);

        let mut encoded_chunk = bitvec::bitvec![u8, Msb0; 0; EVEN_CHUNK_BIT_SIZE];
        encoded_chunk.store_be(encoded_number);

        encoded_chunk
    }

    fn encode_odd_chunk(&self, chunk: &str) -> BitVec<u8, Msb0> {
        let encoded_number = self
            .alphanumeric_encoding_table
            .get(chunk.chars().nth(0).unwrap())
            .unwrap();

        let mut encoded_chunk = bitvec::bitvec![u8, Msb0; 0; ODD_CHUNK_BIT_SIZE];
        encoded_chunk.store_be(encoded_number);

        encoded_chunk
    }
}

impl DataEncoder for AlphanumericDataEncoder {
    fn encode(&self, data: &str) -> BitVec<u8, Msb0> {
        let mut encoded_data: BitVec<u8, Msb0> = BitVec::new();

        for chunk in &data.chars().chunks(CHUNK_SIZE) {
            let mut encoded_chunk = self.encode_chunk(chunk.collect::<String>().as_str());
            encoded_data.append(&mut encoded_chunk);
        }

        encoded_data
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
                    let encoder = AlphanumericDataEncoder::new();
                    let encoded_data = encoder.encode($data);
                    assert_eq!(encoded_data, $expected_encoded_data);
                }
            )+
        };
    }

    test_encode! {
        test_encode_characters: "HELLO WORLD" -> bitvec::bitvec![
            u8, Msb0;
            0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0,
            0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0,
            1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1,
        ],
        test_encode_characters_and_digits: "CHARACTER123AND456DIGIT" -> bitvec::bitvec![
            u8, Msb0;
            0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1,
            1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0,
            0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0,
            1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1,
            0, 1,
        ],
        test_encode_characters_digits_and_symbols: "C H$A%R*A+C-T.E/RS1:23" -> bitvec::bitvec![
            u8, Msb0;
            0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
            1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1,
            0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0,
            1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0,
            0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1,
        ],
        test_encode_even: "AB12" -> bitvec::bitvec![
            u8, Msb0;
            0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1,
        ],
        test_encode_odd: "AB12C" -> bitvec::bitvec![
            u8, Msb0;
            0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1,
            1, 0, 0,
        ]
    }
}

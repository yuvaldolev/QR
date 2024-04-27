use bitvec::{field::BitField, order::Msb0, vec::BitVec};
use itertools::Itertools;

use super::data_encoder::DataEncoder;

const CHUNK_SIZE: usize = 3;

pub struct NumericDataEncoder;

impl NumericDataEncoder {
    pub fn new() -> Self {
        Self
    }

    fn encode_chunk(chunk: &str) -> BitVec<usize, Msb0> {
        let number: u16 = chunk.parse().unwrap();

        let mut encoded_chunk = bitvec::bitvec![
            usize, Msb0;
            0; Self::get_bit_size(number)
        ];
        encoded_chunk.store(number);

        encoded_chunk
    }

    fn get_bit_size(chunk: u16) -> usize {
        let number_of_digits = if 0 == chunk {
            1
        } else {
            ((chunk as f32).log10().floor()) as u32 + 1
        };

        let lower_bound = 10i32.pow(number_of_digits - 1);
        let upper_bound = 10i32.pow(number_of_digits) - 1;

        let number_of_distinct_values = (upper_bound - lower_bound) + 1;

        (number_of_distinct_values as f32).log2().ceil() as usize
    }
}

impl DataEncoder for NumericDataEncoder {
    fn encode(&self, data: &str) -> BitVec<usize, Msb0> {
        let mut encoded_data: BitVec<usize, Msb0> = BitVec::new();

        for chunk in &data.chars().chunks(CHUNK_SIZE) {
            let mut encoded_chunk = Self::encode_chunk(chunk.collect::<String>().as_str());
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
                    let encoder = NumericDataEncoder::new();
                    let encoded_data = encoder.encode($data);
                    assert_eq!(encoded_data, $expected_encoded_data);
                }
            )+
        };
    }

    test_encode! {
        test_encode_full_chunks: "123456" -> bitvec::bitvec![
            usize, Msb0;
            0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0,
        ],
        test_encode_2_digit_last_chunk: "12345" -> bitvec::bitvec![
            usize, Msb0;
            0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1,
        ],
        test_encode_1_digit_last_chunk: "1234" -> bitvec::bitvec![
            usize, Msb0;
            0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0,
        ],
        test_encode_partial_chunks_in_middle: "1230010120001" -> bitvec::bitvec![
            usize, Msb0;
            0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1,
        ]
    }
}

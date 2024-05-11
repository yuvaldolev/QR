use bitvec::{field::BitField, order::Msb0, vec::BitVec};

use crate::{
    data_encoders::DataEncoderFactory, data_encoding::DataEncoding, segment::Segment,
    version::Version,
};

const MODE_INDICATOR_BIT_SIZE: usize = 4;

pub struct SegmentEncoder {
    data_encoder_factory: DataEncoderFactory,
}

impl SegmentEncoder {
    pub fn new() -> Self {
        Self {
            data_encoder_factory: DataEncoderFactory::new(),
        }
    }

    pub fn encode(&self, data: &str, version: &Version, encoding: &DataEncoding) -> Segment {
        let mut segment_data: BitVec<u8, Msb0> = BitVec::new();

        let mut encoded_mode_indicator = self.encode_mode_indicator(encoding);
        segment_data.append(&mut encoded_mode_indicator);

        let mut encoded_character_count_indicator =
            self.encode_character_count_indicator(data.len(), version);
        segment_data.append(&mut encoded_character_count_indicator);

        let data_encoder = self.data_encoder_factory.make(encoding);
        let mut encoded_data = data_encoder.encode(data);
        segment_data.append(&mut encoded_data);

        Segment::new()
    }

    fn encode_mode_indicator(&self, encoding: &DataEncoding) -> BitVec<u8, Msb0> {
        let mut encoded_mode_indicator = bitvec::bitvec![
            u8, Msb0;
            0; MODE_INDICATOR_BIT_SIZE
        ];
        encoded_mode_indicator.store_be(encoding.as_mode_indicator());

        encoded_mode_indicator
    }

    fn encode_character_count_indicator(
        &self,
        data_length: usize,
        version: &Version,
    ) -> BitVec<u8, Msb0> {
        let mut encoded_character_count_indicator = bitvec::bitvec![
            u8, Msb0;
            0; version.get_character_count_indicator_bit_size()
        ];
        encoded_character_count_indicator.store_be(data_length);

        encoded_character_count_indicator
    }
}

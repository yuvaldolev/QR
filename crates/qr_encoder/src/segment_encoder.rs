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

    pub fn encode(&self, segment: &Segment, data: &str, version: &Version) -> BitVec<u8, Msb0> {
        let mut encoded_segment: BitVec<u8, Msb0> = BitVec::new();

        let segment_data = segment.slice(data);

        let mut encoded_mode_indicator = self.encode_mode_indicator(segment.get_encoding());
        encoded_segment.append(&mut encoded_mode_indicator);

        let mut encoded_character_count_indicator = self.encode_character_count_indicator(
            segment_data.len(),
            version,
            segment.get_encoding(),
        );
        encoded_segment.append(&mut encoded_character_count_indicator);

        let data_encoder = self.data_encoder_factory.make(segment.get_encoding());
        let mut encoded_data = data_encoder.encode(segment_data);
        encoded_segment.append(&mut encoded_data);

        encoded_segment
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
        encoding: &DataEncoding,
    ) -> BitVec<u8, Msb0> {
        let mut encoded_character_count_indicator = bitvec::bitvec![
            u8, Msb0;
            0; version.get_character_count_indicator_bit_size(encoding)
        ];
        encoded_character_count_indicator.store_be(data_length);

        encoded_character_count_indicator
    }
}

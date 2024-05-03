use bitvec::{field::BitField, order::Msb0, vec::BitVec};

use crate::{data_encoders::DataEncoderFactory, data_encoding::DataEncoding, segment::Segment};

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

    pub fn encode(&self, data: &str, encoding: &DataEncoding) -> Segment {
        let segment_data: BitVec<u8, Msb0> = BitVec::new();

        let encoded_mode_indicator = self.encode_mode_indicator(encoding);
        segment_data.append(&mut encoded_mode_indicator);

        let data_encoder = self.data_encoder_factory.make(encoding);
        let encoded_data = data_encoder.encode(data);
    }

    fn encode_mode_indicator(&self, encoding: &DataEncoding) -> BitVec<u8, Msb0> {
        let mut encoded_mode_indicator = bitvec::bitvec![
            u8, Msb0;
            0; MODE_INDICATOR_BIT_SIZE
        ];
        encoded_mode_indicator.store_be(encoding.as_mode_indicator());

        encoded_mode_indicator
    }
}

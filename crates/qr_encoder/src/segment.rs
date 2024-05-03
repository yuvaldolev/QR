use bitvec::{order::Msb0, vec::BitVec};

use crate::{
    data_analyzer::DataAnalyzer, data_encoders::DataEncoderFactory, data_encoding::DataEncoding,
};

pub struct Segment {
    encoding: DataEncoding,
    character_count: usize,
    data: BitVec<u8, Msb0>,
}

impl Segment {
    pub fn new(data: &str, encoding: &DataEncoding) -> Self {
        let data_encoder_factory = DataEncoderFactory::new();
        let data_encoder = data_encoder_factory.make(&encoding);
        let encoded_data = data_encoder.encode(data);

        Segment {
            encoding,
            character_count: data.len(),
            data: encoded_data,
        }
    }

    pub fn encode(&self) -> BitVec<u8, Msb0> {}
}

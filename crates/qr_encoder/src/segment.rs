use crate::{data_encoding::DataEncoding, version::Version};

pub struct Segment {
    start: usize,
    end: usize,
    encoding: DataEncoding,
}

impl Segment {
    pub fn new(start: usize, end: usize, encoding: DataEncoding) -> Self {
        Segment {
            encoding,
            start,
            end,
        }
    }

    pub fn slice<'a>(&self, data: &'a str) -> &'a str {
        &data[self.start..self.end]
    }

    pub fn get_encoding(&self) -> &DataEncoding {
        &self.encoding
    }

    pub fn encoded_length(&self, version: &Version) -> usize {
        let character_count = self.end - self.start;

        let mode_indicator_bit_size = version.get_mode_indicator_bit_size();
        let character_count_indicator_bit_size =
            version.get_character_count_indicator_bit_size(&self.encoding);
        let data_bit_size = self.encoding.get_data_bit_size(character_count);

        mode_indicator_bit_size + character_count_indicator_bit_size + data_bit_size
    }
}

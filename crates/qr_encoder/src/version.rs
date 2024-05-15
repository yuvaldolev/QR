use crate::{data_encoding::DataEncoding, ErrorCorrectionLevel};

// This table is copied from ISO/IEC 18004:2006 §6.4.10, Table 7.
const VERSION_ERROR_CORRECTION_DATA_CAPACITY: [[usize; 4]; 40] = [
    // Normal versions
    [152, 128, 104, 72],
    [272, 224, 176, 128],
    [440, 352, 272, 208],
    [640, 512, 384, 288],
    [864, 688, 496, 368],
    [1088, 864, 608, 480],
    [1248, 992, 704, 528],
    [1552, 1232, 880, 688],
    [1856, 1456, 1056, 800],
    [2192, 1728, 1232, 976],
    [2592, 2032, 1440, 1120],
    [2960, 2320, 1648, 1264],
    [3424, 2672, 1952, 1440],
    [3688, 2920, 2088, 1576],
    [4184, 3320, 2360, 1784],
    [4712, 3624, 2600, 2024],
    [5176, 4056, 2936, 2264],
    [5768, 4504, 3176, 2504],
    [6360, 5016, 3560, 2728],
    [6888, 5352, 3880, 3080],
    [7456, 5712, 4096, 3248],
    [8048, 6256, 4544, 3536],
    [8752, 6880, 4912, 3712],
    [9392, 7312, 5312, 4112],
    [10208, 8000, 5744, 4304],
    [10960, 8496, 6032, 4768],
    [11744, 9024, 6464, 5024],
    [12248, 9544, 6968, 5288],
    [13048, 10136, 7288, 5608],
    [13880, 10984, 7880, 5960],
    [14744, 11640, 8264, 6344],
    [15640, 12328, 8920, 6760],
    [16568, 13048, 9368, 7208],
    [17528, 13800, 9848, 7688],
    [18448, 14496, 10288, 7888],
    [19472, 15312, 10832, 8432],
    [20528, 15936, 11408, 8768],
    [21616, 16816, 12016, 9136],
    [22496, 17728, 12656, 9776],
    [23648, 18672, 13328, 10208],
];

pub struct Version {
    number: u8,
}

impl Version {
    pub fn new(number: u8) -> Self {
        Self { number }
    }

    pub fn get_number(&self) -> u8 {
        self.number
    }

    pub fn get_mode_indicator_bit_size(&self) -> usize {
        4
    }

    pub fn get_character_count_indicator_bit_size(&self, encoding: &DataEncoding) -> usize {
        match self.number {
            1..=9 => match encoding {
                DataEncoding::Numeric => 10,
                DataEncoding::Alphanumeric => 9,
                DataEncoding::Byte => 8,
            },
            10..=26 => match encoding {
                DataEncoding::Numeric => 12,
                DataEncoding::Alphanumeric => 11,
                DataEncoding::Byte => 16,
            },
            27..=40 => match encoding {
                DataEncoding::Numeric => 14,
                DataEncoding::Alphanumeric => 13,
                DataEncoding::Byte => 16,
            },
            _ => panic!("invalid version number"),
        }
    }

    pub fn get_data_capacity(&self, error_correction_level: ErrorCorrectionLevel) -> usize {
        VERSION_ERROR_CORRECTION_DATA_CAPACITY[self.number as usize - 1]
            [error_correction_level as usize]
    }
}

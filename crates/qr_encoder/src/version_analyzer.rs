use std::collections::HashMap;

use crate::{data_encoding::DataEncoding, error, ErrorCorrectionLevel};

const NUMBER_OF_SUPPORTED_VERSIONS: usize = 40;

pub struct VersionAnalyzer {
    version_character_capacities: Vec<HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>>,
}

impl VersionAnalyzer {
    pub fn new() -> Self {
        Self {
            version_character_capacities: Self::new_version_character_capacities_table(),
        }
    }

    pub fn analyze(
        &self,
        data_length: usize,
        data_encoding: &DataEncoding,
        error_correction_level: &ErrorCorrectionLevel,
    ) -> error::Result<u8> {
        for (index, version_character_capacities) in
            self.version_character_capacities.iter().enumerate()
        {
            if data_length
                <= *(version_character_capacities
                    .get(error_correction_level)
                    .unwrap()
                    .get(data_encoding)
                    .unwrap())
            {
                return Ok((index + 1) as u8);
            }
        }

        Err(error::Error::DataTooLong(data_length))
    }

    fn new_version_character_capacities_table(
    ) -> Vec<HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>> {
        let mut version_character_capacities: Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        > = vec![HashMap::new(); NUMBER_OF_SUPPORTED_VERSIONS];

        Self::insert_version_1_character_capacities(&mut version_character_capacities);
        Self::insert_version_2_character_capacities(&mut version_character_capacities);
        Self::insert_version_3_character_capacities(&mut version_character_capacities);
        Self::insert_version_4_character_capacities(&mut version_character_capacities);
        Self::insert_version_5_character_capacities(&mut version_character_capacities);
        Self::insert_version_6_character_capacities(&mut version_character_capacities);
        Self::insert_version_7_character_capacities(&mut version_character_capacities);
        Self::insert_version_8_character_capacities(&mut version_character_capacities);
        Self::insert_version_9_character_capacities(&mut version_character_capacities);
        Self::insert_version_10_character_capacities(&mut version_character_capacities);
        Self::insert_version_11_character_capacities(&mut version_character_capacities);
        Self::insert_version_12_character_capacities(&mut version_character_capacities);
        Self::insert_version_13_character_capacities(&mut version_character_capacities);
        Self::insert_version_14_character_capacities(&mut version_character_capacities);
        Self::insert_version_15_character_capacities(&mut version_character_capacities);
        Self::insert_version_16_character_capacities(&mut version_character_capacities);
        Self::insert_version_17_character_capacities(&mut version_character_capacities);
        Self::insert_version_18_character_capacities(&mut version_character_capacities);
        Self::insert_version_19_character_capacities(&mut version_character_capacities);
        Self::insert_version_20_character_capacities(&mut version_character_capacities);
        Self::insert_version_21_character_capacities(&mut version_character_capacities);
        Self::insert_version_22_character_capacities(&mut version_character_capacities);
        Self::insert_version_23_character_capacities(&mut version_character_capacities);
        Self::insert_version_24_character_capacities(&mut version_character_capacities);
        Self::insert_version_25_character_capacities(&mut version_character_capacities);
        Self::insert_version_26_character_capacities(&mut version_character_capacities);
        Self::insert_version_27_character_capacities(&mut version_character_capacities);
        Self::insert_version_28_character_capacities(&mut version_character_capacities);
        Self::insert_version_29_character_capacities(&mut version_character_capacities);
        Self::insert_version_30_character_capacities(&mut version_character_capacities);
        Self::insert_version_31_character_capacities(&mut version_character_capacities);
        Self::insert_version_32_character_capacities(&mut version_character_capacities);
        Self::insert_version_33_character_capacities(&mut version_character_capacities);
        Self::insert_version_34_character_capacities(&mut version_character_capacities);
        Self::insert_version_35_character_capacities(&mut version_character_capacities);
        Self::insert_version_36_character_capacities(&mut version_character_capacities);
        Self::insert_version_37_character_capacities(&mut version_character_capacities);
        Self::insert_version_38_character_capacities(&mut version_character_capacities);
        Self::insert_version_39_character_capacities(&mut version_character_capacities);
        Self::insert_version_40_character_capacities(&mut version_character_capacities);

        version_character_capacities
    }

    fn insert_version_1_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            1,
            [
                (DataEncoding::Numeric, 41),
                (DataEncoding::Alphanumeric, 25),
                (DataEncoding::Byte, 17),
            ],
            [
                (DataEncoding::Numeric, 34),
                (DataEncoding::Alphanumeric, 20),
                (DataEncoding::Byte, 14),
            ],
            [
                (DataEncoding::Numeric, 27),
                (DataEncoding::Alphanumeric, 16),
                (DataEncoding::Byte, 11),
            ],
            [
                (DataEncoding::Numeric, 17),
                (DataEncoding::Alphanumeric, 10),
                (DataEncoding::Byte, 7),
            ],
        );
    }

    fn insert_version_2_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            2,
            [
                (DataEncoding::Numeric, 77),
                (DataEncoding::Alphanumeric, 47),
                (DataEncoding::Byte, 32),
            ],
            [
                (DataEncoding::Numeric, 63),
                (DataEncoding::Alphanumeric, 38),
                (DataEncoding::Byte, 26),
            ],
            [
                (DataEncoding::Numeric, 48),
                (DataEncoding::Alphanumeric, 29),
                (DataEncoding::Byte, 20),
            ],
            [
                (DataEncoding::Numeric, 34),
                (DataEncoding::Alphanumeric, 20),
                (DataEncoding::Byte, 14),
            ],
        );
    }

    fn insert_version_3_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            3,
            [
                (DataEncoding::Numeric, 127),
                (DataEncoding::Alphanumeric, 77),
                (DataEncoding::Byte, 53),
            ],
            [
                (DataEncoding::Numeric, 101),
                (DataEncoding::Alphanumeric, 61),
                (DataEncoding::Byte, 42),
            ],
            [
                (DataEncoding::Numeric, 77),
                (DataEncoding::Alphanumeric, 47),
                (DataEncoding::Byte, 32),
            ],
            [
                (DataEncoding::Numeric, 58),
                (DataEncoding::Alphanumeric, 35),
                (DataEncoding::Byte, 24),
            ],
        );
    }

    fn insert_version_4_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            4,
            [
                (DataEncoding::Numeric, 187),
                (DataEncoding::Alphanumeric, 114),
                (DataEncoding::Byte, 78),
            ],
            [
                (DataEncoding::Numeric, 149),
                (DataEncoding::Alphanumeric, 90),
                (DataEncoding::Byte, 62),
            ],
            [
                (DataEncoding::Numeric, 111),
                (DataEncoding::Alphanumeric, 67),
                (DataEncoding::Byte, 46),
            ],
            [
                (DataEncoding::Numeric, 82),
                (DataEncoding::Alphanumeric, 50),
                (DataEncoding::Byte, 34),
            ],
        );
    }

    fn insert_version_5_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            5,
            [
                (DataEncoding::Numeric, 255),
                (DataEncoding::Alphanumeric, 154),
                (DataEncoding::Byte, 106),
            ],
            [
                (DataEncoding::Numeric, 202),
                (DataEncoding::Alphanumeric, 122),
                (DataEncoding::Byte, 84),
            ],
            [
                (DataEncoding::Numeric, 144),
                (DataEncoding::Alphanumeric, 87),
                (DataEncoding::Byte, 60),
            ],
            [
                (DataEncoding::Numeric, 106),
                (DataEncoding::Alphanumeric, 64),
                (DataEncoding::Byte, 44),
            ],
        );
    }

    fn insert_version_6_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            6,
            [
                (DataEncoding::Numeric, 322),
                (DataEncoding::Alphanumeric, 195),
                (DataEncoding::Byte, 134),
            ],
            [
                (DataEncoding::Numeric, 255),
                (DataEncoding::Alphanumeric, 154),
                (DataEncoding::Byte, 106),
            ],
            [
                (DataEncoding::Numeric, 178),
                (DataEncoding::Alphanumeric, 108),
                (DataEncoding::Byte, 74),
            ],
            [
                (DataEncoding::Numeric, 139),
                (DataEncoding::Alphanumeric, 84),
                (DataEncoding::Byte, 58),
            ],
        );
    }

    fn insert_version_7_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            7,
            [
                (DataEncoding::Numeric, 370),
                (DataEncoding::Alphanumeric, 224),
                (DataEncoding::Byte, 154),
            ],
            [
                (DataEncoding::Numeric, 293),
                (DataEncoding::Alphanumeric, 178),
                (DataEncoding::Byte, 122),
            ],
            [
                (DataEncoding::Numeric, 207),
                (DataEncoding::Alphanumeric, 125),
                (DataEncoding::Byte, 86),
            ],
            [
                (DataEncoding::Numeric, 154),
                (DataEncoding::Alphanumeric, 93),
                (DataEncoding::Byte, 64),
            ],
        );
    }

    fn insert_version_8_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            8,
            [
                (DataEncoding::Numeric, 461),
                (DataEncoding::Alphanumeric, 279),
                (DataEncoding::Byte, 192),
            ],
            [
                (DataEncoding::Numeric, 365),
                (DataEncoding::Alphanumeric, 221),
                (DataEncoding::Byte, 152),
            ],
            [
                (DataEncoding::Numeric, 259),
                (DataEncoding::Alphanumeric, 157),
                (DataEncoding::Byte, 108),
            ],
            [
                (DataEncoding::Numeric, 202),
                (DataEncoding::Alphanumeric, 122),
                (DataEncoding::Byte, 84),
            ],
        );
    }

    fn insert_version_9_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            9,
            [
                (DataEncoding::Numeric, 552),
                (DataEncoding::Alphanumeric, 335),
                (DataEncoding::Byte, 230),
            ],
            [
                (DataEncoding::Numeric, 432),
                (DataEncoding::Alphanumeric, 262),
                (DataEncoding::Byte, 180),
            ],
            [
                (DataEncoding::Numeric, 312),
                (DataEncoding::Alphanumeric, 189),
                (DataEncoding::Byte, 130),
            ],
            [
                (DataEncoding::Numeric, 235),
                (DataEncoding::Alphanumeric, 143),
                (DataEncoding::Byte, 98),
            ],
        );
    }

    fn insert_version_10_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            10,
            [
                (DataEncoding::Numeric, 652),
                (DataEncoding::Alphanumeric, 395),
                (DataEncoding::Byte, 271),
            ],
            [
                (DataEncoding::Numeric, 513),
                (DataEncoding::Alphanumeric, 311),
                (DataEncoding::Byte, 213),
            ],
            [
                (DataEncoding::Numeric, 364),
                (DataEncoding::Alphanumeric, 221),
                (DataEncoding::Byte, 151),
            ],
            [
                (DataEncoding::Numeric, 288),
                (DataEncoding::Alphanumeric, 174),
                (DataEncoding::Byte, 119),
            ],
        );
    }

    fn insert_version_11_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            11,
            [
                (DataEncoding::Numeric, 772),
                (DataEncoding::Alphanumeric, 468),
                (DataEncoding::Byte, 321),
            ],
            [
                (DataEncoding::Numeric, 604),
                (DataEncoding::Alphanumeric, 366),
                (DataEncoding::Byte, 251),
            ],
            [
                (DataEncoding::Numeric, 427),
                (DataEncoding::Alphanumeric, 259),
                (DataEncoding::Byte, 177),
            ],
            [
                (DataEncoding::Numeric, 331),
                (DataEncoding::Alphanumeric, 200),
                (DataEncoding::Byte, 137),
            ],
        );
    }

    fn insert_version_12_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            12,
            [
                (DataEncoding::Numeric, 883),
                (DataEncoding::Alphanumeric, 535),
                (DataEncoding::Byte, 367),
            ],
            [
                (DataEncoding::Numeric, 691),
                (DataEncoding::Alphanumeric, 419),
                (DataEncoding::Byte, 287),
            ],
            [
                (DataEncoding::Numeric, 489),
                (DataEncoding::Alphanumeric, 296),
                (DataEncoding::Byte, 203),
            ],
            [
                (DataEncoding::Numeric, 374),
                (DataEncoding::Alphanumeric, 227),
                (DataEncoding::Byte, 155),
            ],
        );
    }

    fn insert_version_13_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            13,
            [
                (DataEncoding::Numeric, 1022),
                (DataEncoding::Alphanumeric, 619),
                (DataEncoding::Byte, 425),
            ],
            [
                (DataEncoding::Numeric, 796),
                (DataEncoding::Alphanumeric, 483),
                (DataEncoding::Byte, 331),
            ],
            [
                (DataEncoding::Numeric, 580),
                (DataEncoding::Alphanumeric, 354),
                (DataEncoding::Byte, 243),
            ],
            [
                (DataEncoding::Numeric, 427),
                (DataEncoding::Alphanumeric, 259),
                (DataEncoding::Byte, 177),
            ],
        );
    }

    fn insert_version_14_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            14,
            [
                (DataEncoding::Numeric, 1101),
                (DataEncoding::Alphanumeric, 667),
                (DataEncoding::Byte, 458),
            ],
            [
                (DataEncoding::Numeric, 871),
                (DataEncoding::Alphanumeric, 528),
                (DataEncoding::Byte, 362),
            ],
            [
                (DataEncoding::Numeric, 621),
                (DataEncoding::Alphanumeric, 376),
                (DataEncoding::Byte, 258),
            ],
            [
                (DataEncoding::Numeric, 468),
                (DataEncoding::Alphanumeric, 283),
                (DataEncoding::Byte, 194),
            ],
        );
    }

    fn insert_version_15_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            15,
            [
                (DataEncoding::Numeric, 1250),
                (DataEncoding::Alphanumeric, 758),
                (DataEncoding::Byte, 520),
            ],
            [
                (DataEncoding::Numeric, 991),
                (DataEncoding::Alphanumeric, 600),
                (DataEncoding::Byte, 412),
            ],
            [
                (DataEncoding::Numeric, 703),
                (DataEncoding::Alphanumeric, 426),
                (DataEncoding::Byte, 292),
            ],
            [
                (DataEncoding::Numeric, 530),
                (DataEncoding::Alphanumeric, 321),
                (DataEncoding::Byte, 220),
            ],
        );
    }

    fn insert_version_16_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            16,
            [
                (DataEncoding::Numeric, 1408),
                (DataEncoding::Alphanumeric, 854),
                (DataEncoding::Byte, 586),
            ],
            [
                (DataEncoding::Numeric, 1082),
                (DataEncoding::Alphanumeric, 656),
                (DataEncoding::Byte, 450),
            ],
            [
                (DataEncoding::Numeric, 775),
                (DataEncoding::Alphanumeric, 470),
                (DataEncoding::Byte, 322),
            ],
            [
                (DataEncoding::Numeric, 602),
                (DataEncoding::Alphanumeric, 365),
                (DataEncoding::Byte, 250),
            ],
        );
    }

    fn insert_version_17_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            17,
            [
                (DataEncoding::Numeric, 1548),
                (DataEncoding::Alphanumeric, 938),
                (DataEncoding::Byte, 644),
            ],
            [
                (DataEncoding::Numeric, 1212),
                (DataEncoding::Alphanumeric, 734),
                (DataEncoding::Byte, 504),
            ],
            [
                (DataEncoding::Numeric, 876),
                (DataEncoding::Alphanumeric, 531),
                (DataEncoding::Byte, 364),
            ],
            [
                (DataEncoding::Numeric, 674),
                (DataEncoding::Alphanumeric, 408),
                (DataEncoding::Byte, 280),
            ],
        );
    }

    fn insert_version_18_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            18,
            [
                (DataEncoding::Numeric, 1725),
                (DataEncoding::Alphanumeric, 1046),
                (DataEncoding::Byte, 718),
            ],
            [
                (DataEncoding::Numeric, 1346),
                (DataEncoding::Alphanumeric, 816),
                (DataEncoding::Byte, 560),
            ],
            [
                (DataEncoding::Numeric, 948),
                (DataEncoding::Alphanumeric, 574),
                (DataEncoding::Byte, 394),
            ],
            [
                (DataEncoding::Numeric, 746),
                (DataEncoding::Alphanumeric, 452),
                (DataEncoding::Byte, 310),
            ],
        );
    }

    fn insert_version_19_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            19,
            [
                (DataEncoding::Numeric, 1903),
                (DataEncoding::Alphanumeric, 1153),
                (DataEncoding::Byte, 792),
            ],
            [
                (DataEncoding::Numeric, 1500),
                (DataEncoding::Alphanumeric, 909),
                (DataEncoding::Byte, 624),
            ],
            [
                (DataEncoding::Numeric, 1063),
                (DataEncoding::Alphanumeric, 644),
                (DataEncoding::Byte, 442),
            ],
            [
                (DataEncoding::Numeric, 813),
                (DataEncoding::Alphanumeric, 493),
                (DataEncoding::Byte, 338),
            ],
        );
    }

    fn insert_version_20_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            20,
            [
                (DataEncoding::Numeric, 2061),
                (DataEncoding::Alphanumeric, 1249),
                (DataEncoding::Byte, 858),
            ],
            [
                (DataEncoding::Numeric, 1600),
                (DataEncoding::Alphanumeric, 970),
                (DataEncoding::Byte, 666),
            ],
            [
                (DataEncoding::Numeric, 1159),
                (DataEncoding::Alphanumeric, 702),
                (DataEncoding::Byte, 482),
            ],
            [
                (DataEncoding::Numeric, 919),
                (DataEncoding::Alphanumeric, 557),
                (DataEncoding::Byte, 382),
            ],
        );
    }

    fn insert_version_21_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            21,
            [
                (DataEncoding::Numeric, 2232),
                (DataEncoding::Alphanumeric, 1352),
                (DataEncoding::Byte, 929),
            ],
            [
                (DataEncoding::Numeric, 1708),
                (DataEncoding::Alphanumeric, 1035),
                (DataEncoding::Byte, 711),
            ],
            [
                (DataEncoding::Numeric, 1224),
                (DataEncoding::Alphanumeric, 742),
                (DataEncoding::Byte, 509),
            ],
            [
                (DataEncoding::Numeric, 969),
                (DataEncoding::Alphanumeric, 587),
                (DataEncoding::Byte, 403),
            ],
        );
    }

    fn insert_version_22_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            22,
            [
                (DataEncoding::Numeric, 2409),
                (DataEncoding::Alphanumeric, 1460),
                (DataEncoding::Byte, 1003),
            ],
            [
                (DataEncoding::Numeric, 1872),
                (DataEncoding::Alphanumeric, 1134),
                (DataEncoding::Byte, 779),
            ],
            [
                (DataEncoding::Numeric, 1358),
                (DataEncoding::Alphanumeric, 823),
                (DataEncoding::Byte, 565),
            ],
            [
                (DataEncoding::Numeric, 1056),
                (DataEncoding::Alphanumeric, 640),
                (DataEncoding::Byte, 439),
            ],
        );
    }

    fn insert_version_23_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            23,
            [
                (DataEncoding::Numeric, 2620),
                (DataEncoding::Alphanumeric, 1588),
                (DataEncoding::Byte, 1091),
            ],
            [
                (DataEncoding::Numeric, 2059),
                (DataEncoding::Alphanumeric, 1248),
                (DataEncoding::Byte, 857),
            ],
            [
                (DataEncoding::Numeric, 1468),
                (DataEncoding::Alphanumeric, 890),
                (DataEncoding::Byte, 611),
            ],
            [
                (DataEncoding::Numeric, 1108),
                (DataEncoding::Alphanumeric, 672),
                (DataEncoding::Byte, 461),
            ],
        );
    }

    fn insert_version_24_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            24,
            [
                (DataEncoding::Numeric, 2812),
                (DataEncoding::Alphanumeric, 1704),
                (DataEncoding::Byte, 1171),
            ],
            [
                (DataEncoding::Numeric, 2188),
                (DataEncoding::Alphanumeric, 1326),
                (DataEncoding::Byte, 911),
            ],
            [
                (DataEncoding::Numeric, 1588),
                (DataEncoding::Alphanumeric, 963),
                (DataEncoding::Byte, 661),
            ],
            [
                (DataEncoding::Numeric, 1228),
                (DataEncoding::Alphanumeric, 744),
                (DataEncoding::Byte, 511),
            ],
        );
    }

    fn insert_version_25_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            25,
            [
                (DataEncoding::Numeric, 3057),
                (DataEncoding::Alphanumeric, 1853),
                (DataEncoding::Byte, 1273),
            ],
            [
                (DataEncoding::Numeric, 2395),
                (DataEncoding::Alphanumeric, 1451),
                (DataEncoding::Byte, 997),
            ],
            [
                (DataEncoding::Numeric, 1718),
                (DataEncoding::Alphanumeric, 1041),
                (DataEncoding::Byte, 715),
            ],
            [
                (DataEncoding::Numeric, 1286),
                (DataEncoding::Alphanumeric, 779),
                (DataEncoding::Byte, 535),
            ],
        );
    }

    fn insert_version_26_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            26,
            [
                (DataEncoding::Numeric, 3283),
                (DataEncoding::Alphanumeric, 1990),
                (DataEncoding::Byte, 1367),
            ],
            [
                (DataEncoding::Numeric, 2544),
                (DataEncoding::Alphanumeric, 1542),
                (DataEncoding::Byte, 1059),
            ],
            [
                (DataEncoding::Numeric, 1804),
                (DataEncoding::Alphanumeric, 1094),
                (DataEncoding::Byte, 751),
            ],
            [
                (DataEncoding::Numeric, 1425),
                (DataEncoding::Alphanumeric, 864),
                (DataEncoding::Byte, 593),
            ],
        );
    }

    fn insert_version_27_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            27,
            [
                (DataEncoding::Numeric, 3517),
                (DataEncoding::Alphanumeric, 2132),
                (DataEncoding::Byte, 1465),
            ],
            [
                (DataEncoding::Numeric, 2701),
                (DataEncoding::Alphanumeric, 1637),
                (DataEncoding::Byte, 1125),
            ],
            [
                (DataEncoding::Numeric, 1933),
                (DataEncoding::Alphanumeric, 1172),
                (DataEncoding::Byte, 805),
            ],
            [
                (DataEncoding::Numeric, 1501),
                (DataEncoding::Alphanumeric, 910),
                (DataEncoding::Byte, 625),
            ],
        );
    }

    fn insert_version_28_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            28,
            [
                (DataEncoding::Numeric, 3669),
                (DataEncoding::Alphanumeric, 2223),
                (DataEncoding::Byte, 1528),
            ],
            [
                (DataEncoding::Numeric, 2857),
                (DataEncoding::Alphanumeric, 1732),
                (DataEncoding::Byte, 1190),
            ],
            [
                (DataEncoding::Numeric, 2085),
                (DataEncoding::Alphanumeric, 1263),
                (DataEncoding::Byte, 868),
            ],
            [
                (DataEncoding::Numeric, 1581),
                (DataEncoding::Alphanumeric, 958),
                (DataEncoding::Byte, 658),
            ],
        );
    }

    fn insert_version_29_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            29,
            [
                (DataEncoding::Numeric, 3909),
                (DataEncoding::Alphanumeric, 2369),
                (DataEncoding::Byte, 1628),
            ],
            [
                (DataEncoding::Numeric, 3035),
                (DataEncoding::Alphanumeric, 1839),
                (DataEncoding::Byte, 1264),
            ],
            [
                (DataEncoding::Numeric, 2181),
                (DataEncoding::Alphanumeric, 1322),
                (DataEncoding::Byte, 908),
            ],
            [
                (DataEncoding::Numeric, 1677),
                (DataEncoding::Alphanumeric, 1016),
                (DataEncoding::Byte, 698),
            ],
        );
    }

    fn insert_version_30_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            30,
            [
                (DataEncoding::Numeric, 4158),
                (DataEncoding::Alphanumeric, 2520),
                (DataEncoding::Byte, 1732),
            ],
            [
                (DataEncoding::Numeric, 3289),
                (DataEncoding::Alphanumeric, 1994),
                (DataEncoding::Byte, 1370),
            ],
            [
                (DataEncoding::Numeric, 2358),
                (DataEncoding::Alphanumeric, 1429),
                (DataEncoding::Byte, 982),
            ],
            [
                (DataEncoding::Numeric, 1782),
                (DataEncoding::Alphanumeric, 1080),
                (DataEncoding::Byte, 742),
            ],
        );
    }

    fn insert_version_31_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            31,
            [
                (DataEncoding::Numeric, 4417),
                (DataEncoding::Alphanumeric, 2677),
                (DataEncoding::Byte, 1840),
            ],
            [
                (DataEncoding::Numeric, 3486),
                (DataEncoding::Alphanumeric, 2113),
                (DataEncoding::Byte, 1452),
            ],
            [
                (DataEncoding::Numeric, 2473),
                (DataEncoding::Alphanumeric, 1499),
                (DataEncoding::Byte, 1030),
            ],
            [
                (DataEncoding::Numeric, 1897),
                (DataEncoding::Alphanumeric, 1150),
                (DataEncoding::Byte, 790),
            ],
        );
    }

    fn insert_version_32_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            32,
            [
                (DataEncoding::Numeric, 4686),
                (DataEncoding::Alphanumeric, 2840),
                (DataEncoding::Byte, 1952),
            ],
            [
                (DataEncoding::Numeric, 3693),
                (DataEncoding::Alphanumeric, 2238),
                (DataEncoding::Byte, 1538),
            ],
            [
                (DataEncoding::Numeric, 2670),
                (DataEncoding::Alphanumeric, 1618),
                (DataEncoding::Byte, 1112),
            ],
            [
                (DataEncoding::Numeric, 2022),
                (DataEncoding::Alphanumeric, 1226),
                (DataEncoding::Byte, 842),
            ],
        );
    }

    fn insert_version_33_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            33,
            [
                (DataEncoding::Numeric, 4965),
                (DataEncoding::Alphanumeric, 3009),
                (DataEncoding::Byte, 2068),
            ],
            [
                (DataEncoding::Numeric, 3909),
                (DataEncoding::Alphanumeric, 2369),
                (DataEncoding::Byte, 1628),
            ],
            [
                (DataEncoding::Numeric, 2805),
                (DataEncoding::Alphanumeric, 1700),
                (DataEncoding::Byte, 1168),
            ],
            [
                (DataEncoding::Numeric, 2157),
                (DataEncoding::Alphanumeric, 1307),
                (DataEncoding::Byte, 898),
            ],
        );
    }

    fn insert_version_34_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            34,
            [
                (DataEncoding::Numeric, 5253),
                (DataEncoding::Alphanumeric, 3183),
                (DataEncoding::Byte, 2188),
            ],
            [
                (DataEncoding::Numeric, 4134),
                (DataEncoding::Alphanumeric, 2506),
                (DataEncoding::Byte, 1722),
            ],
            [
                (DataEncoding::Numeric, 2949),
                (DataEncoding::Alphanumeric, 1787),
                (DataEncoding::Byte, 1228),
            ],
            [
                (DataEncoding::Numeric, 2301),
                (DataEncoding::Alphanumeric, 1394),
                (DataEncoding::Byte, 958),
            ],
        );
    }

    fn insert_version_35_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            35,
            [
                (DataEncoding::Numeric, 5529),
                (DataEncoding::Alphanumeric, 3351),
                (DataEncoding::Byte, 2303),
            ],
            [
                (DataEncoding::Numeric, 4343),
                (DataEncoding::Alphanumeric, 2632),
                (DataEncoding::Byte, 1809),
            ],
            [
                (DataEncoding::Numeric, 3081),
                (DataEncoding::Alphanumeric, 1867),
                (DataEncoding::Byte, 1283),
            ],
            [
                (DataEncoding::Numeric, 2361),
                (DataEncoding::Alphanumeric, 1431),
                (DataEncoding::Byte, 983),
            ],
        );
    }

    fn insert_version_36_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            36,
            [
                (DataEncoding::Numeric, 5836),
                (DataEncoding::Alphanumeric, 3537),
                (DataEncoding::Byte, 2431),
            ],
            [
                (DataEncoding::Numeric, 4588),
                (DataEncoding::Alphanumeric, 2780),
                (DataEncoding::Byte, 1911),
            ],
            [
                (DataEncoding::Numeric, 3244),
                (DataEncoding::Alphanumeric, 1966),
                (DataEncoding::Byte, 1351),
            ],
            [
                (DataEncoding::Numeric, 2524),
                (DataEncoding::Alphanumeric, 1530),
                (DataEncoding::Byte, 1051),
            ],
        );
    }

    fn insert_version_37_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            37,
            [
                (DataEncoding::Numeric, 6153),
                (DataEncoding::Alphanumeric, 3729),
                (DataEncoding::Byte, 2563),
            ],
            [
                (DataEncoding::Numeric, 4775),
                (DataEncoding::Alphanumeric, 2894),
                (DataEncoding::Byte, 1989),
            ],
            [
                (DataEncoding::Numeric, 3417),
                (DataEncoding::Alphanumeric, 2071),
                (DataEncoding::Byte, 1423),
            ],
            [
                (DataEncoding::Numeric, 2625),
                (DataEncoding::Alphanumeric, 1591),
                (DataEncoding::Byte, 1093),
            ],
        );
    }

    fn insert_version_38_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            38,
            [
                (DataEncoding::Numeric, 6479),
                (DataEncoding::Alphanumeric, 3927),
                (DataEncoding::Byte, 2699),
            ],
            [
                (DataEncoding::Numeric, 5039),
                (DataEncoding::Alphanumeric, 3054),
                (DataEncoding::Byte, 2099),
            ],
            [
                (DataEncoding::Numeric, 3599),
                (DataEncoding::Alphanumeric, 2181),
                (DataEncoding::Byte, 1499),
            ],
            [
                (DataEncoding::Numeric, 2735),
                (DataEncoding::Alphanumeric, 1658),
                (DataEncoding::Byte, 1139),
            ],
        );
    }

    fn insert_version_39_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            39,
            [
                (DataEncoding::Numeric, 6743),
                (DataEncoding::Alphanumeric, 4087),
                (DataEncoding::Byte, 2809),
            ],
            [
                (DataEncoding::Numeric, 5313),
                (DataEncoding::Alphanumeric, 3220),
                (DataEncoding::Byte, 2213),
            ],
            [
                (DataEncoding::Numeric, 3791),
                (DataEncoding::Alphanumeric, 2298),
                (DataEncoding::Byte, 1579),
            ],
            [
                (DataEncoding::Numeric, 2927),
                (DataEncoding::Alphanumeric, 1774),
                (DataEncoding::Byte, 1219),
            ],
        );
    }

    fn insert_version_40_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
    ) {
        Self::insert_version_character_capacities(
            version_character_capacities,
            40,
            [
                (DataEncoding::Numeric, 7089),
                (DataEncoding::Alphanumeric, 4296),
                (DataEncoding::Byte, 2953),
            ],
            [
                (DataEncoding::Numeric, 5596),
                (DataEncoding::Alphanumeric, 3391),
                (DataEncoding::Byte, 2331),
            ],
            [
                (DataEncoding::Numeric, 3993),
                (DataEncoding::Alphanumeric, 2420),
                (DataEncoding::Byte, 1663),
            ],
            [
                (DataEncoding::Numeric, 3057),
                (DataEncoding::Alphanumeric, 1852),
                (DataEncoding::Byte, 1273),
            ],
        );
    }

    fn insert_version_character_capacities(
        version_character_capacities: &mut Vec<
            HashMap<ErrorCorrectionLevel, HashMap<DataEncoding, usize>>,
        >,
        version: usize,
        low: [(DataEncoding, usize); 3],
        medium: [(DataEncoding, usize); 3],
        quartile: [(DataEncoding, usize); 3],
        high: [(DataEncoding, usize); 3],
    ) {
        version_character_capacities[version - 1]
            .insert(ErrorCorrectionLevel::Low, HashMap::from(low));
        version_character_capacities[version - 1]
            .insert(ErrorCorrectionLevel::Medium, HashMap::from(medium));
        version_character_capacities[version - 1]
            .insert(ErrorCorrectionLevel::Quartile, HashMap::from(quartile));
        version_character_capacities[version - 1]
            .insert(ErrorCorrectionLevel::High, HashMap::from(high));
    }
}

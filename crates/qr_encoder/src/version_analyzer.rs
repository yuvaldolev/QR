use std::collections::HashMap;

use crate::{data_encoding::DataEncoding, ErrorCorrectionLevel};

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

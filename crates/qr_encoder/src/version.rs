pub struct Version {
    number: u8,
}

impl Version {
    pub fn new(number: u8) -> Self {
        Self { number }
    }

    pub fn get_character_count_indicator_bit_size(&self) -> usize {
        match self.number {
            1..=9 => 10,
            10..=26 => 12,
            27..=40 => 14,
            _ => panic!("invalid version number"),
        }
    }
}

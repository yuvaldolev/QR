use std::collections::HashMap;

pub struct AlphanumericEncodingTable {
    table: HashMap<char, u8>,
}

impl AlphanumericEncodingTable {
    pub fn new() -> Self {
        let mut table: HashMap<char, u8> = HashMap::new();

        table.insert('0', 0);
        table.insert('1', 1);
        table.insert('2', 2);
        table.insert('3', 3);
        table.insert('4', 4);
        table.insert('5', 5);
        table.insert('6', 6);
        table.insert('7', 7);
        table.insert('8', 8);
        table.insert('9', 9);
        table.insert('A', 10);
        table.insert('B', 11);
        table.insert('C', 12);
        table.insert('D', 13);
        table.insert('E', 14);
        table.insert('F', 15);
        table.insert('G', 16);
        table.insert('H', 17);
        table.insert('I', 18);
        table.insert('J', 19);
        table.insert('K', 20);
        table.insert('L', 21);
        table.insert('M', 22);
        table.insert('N', 23);
        table.insert('O', 24);
        table.insert('P', 25);
        table.insert('Q', 26);
        table.insert('R', 27);
        table.insert('S', 28);
        table.insert('T', 29);
        table.insert('U', 30);
        table.insert('V', 31);
        table.insert('W', 32);
        table.insert('X', 33);
        table.insert('Y', 34);
        table.insert('Z', 35);
        table.insert(' ', 36);
        table.insert('$', 37);
        table.insert('%', 38);
        table.insert('*', 39);
        table.insert('+', 40);
        table.insert('-', 41);
        table.insert('.', 42);
        table.insert('/', 43);
        table.insert(':', 44);

        Self { table }
    }

    pub fn contains(&self, key: char) -> bool {
        self.table.contains_key(&key)
    }

    pub fn get(&self, key: char) -> qr_error::Result<u8> {
        self.table
            .get(&key)
            .ok_or(qr_error::Error::InvalidAlphanumericEncodingTableKey(key))
            .copied()
    }
}

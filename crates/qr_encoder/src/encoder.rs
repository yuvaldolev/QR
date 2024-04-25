pub struct Encoder;

impl Encoder {
    pub fn new() -> Self {
        Self
    }

    pub fn encode(&self, data: &str) {
        println!("Encoding data: {data}");
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

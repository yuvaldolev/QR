pub trait DataEncoder {
    fn encode(&self, data: &str) -> Vec<u8>;
}

use bitvec::{order::Msb0, vec::BitVec};

pub trait DataEncoder {
    fn encode(&self, data: &str) -> BitVec<usize, Msb0>;
}

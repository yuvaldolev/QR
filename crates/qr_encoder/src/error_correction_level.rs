#[derive(Copy, Clone, Eq, Hash, PartialEq)]
#[repr(usize)]
pub enum ErrorCorrectionLevel {
    Low = 0,
    Medium = 1,
    Quartile = 2,
    High = 3,
}

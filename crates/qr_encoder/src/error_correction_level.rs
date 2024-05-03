#[derive(Clone, Eq, Hash, PartialEq)]
pub enum ErrorCorrectionLevel {
    Low,
    Medium,
    Quartile,
    High,
}

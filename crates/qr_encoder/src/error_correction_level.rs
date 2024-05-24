use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[repr(usize)]
pub enum ErrorCorrectionLevel {
    Low = 0,
    Medium = 1,
    Quartile = 2,
    High = 3,
}

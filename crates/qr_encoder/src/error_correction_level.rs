use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[repr(usize)]
pub enum ErrorCorrectionLevel {
    Low = 0,
    Medium = 1,
    Quartile = 2,
    High = 3,
}

impl Display for ErrorCorrectionLevel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ErrorCorrectionLevel::Low => write!(f, "L"),
            ErrorCorrectionLevel::Medium => write!(f, "M"),
            ErrorCorrectionLevel::Quartile => write!(f, "Q"),
            ErrorCorrectionLevel::High => write!(f, "H"),
        }
    }
}

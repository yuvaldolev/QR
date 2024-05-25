use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ErrorCorrectionLevel;

#[derive(Deserialize, Serialize)]
pub struct Request {
    id: u128,
    data: String,
    error_correction_level: ErrorCorrectionLevel,
}

impl Request {
    pub fn new(data: String, error_correction_level: ErrorCorrectionLevel) -> Self {
        Self {
            id: Uuid::new_v4().as_u128(),
            data,
            error_correction_level,
        }
    }

    pub fn get_id(&self) -> u128 {
        self.id
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Request {{ id: {}, data: {}, error_correction_level: {} }}",
            self.id, self.data, self.error_correction_level
        )
    }
}

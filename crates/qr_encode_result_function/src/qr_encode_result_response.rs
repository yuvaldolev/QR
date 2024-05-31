use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QrEncodeResultResponse {
    data: String,
}

impl QrEncodeResultResponse {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

impl Display for QrEncodeResultResponse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "QrEncodeResultResponse {{ data: {} }}", self.data)
    }
}

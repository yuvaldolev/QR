use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QrEncodeResultOutput {
    data: String,
}

impl QrEncodeResultOutput {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

impl Display for QrEncodeResultOutput {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "QrEncodeResultOutput {{ data: {} }}", self.data)
    }
}

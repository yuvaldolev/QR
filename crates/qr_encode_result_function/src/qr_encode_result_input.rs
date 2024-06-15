use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QrEncodeResultInput {
    id: String,
    data: String,
}

impl QrEncodeResultInput {
    pub fn new(id: String, data: String) -> Self {
        Self { id, data }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }
}

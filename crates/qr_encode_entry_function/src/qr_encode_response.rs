use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct QrEncodeResponse {
    request_id: String,
}

impl QrEncodeResponse {
    pub fn new(request_id: String) -> Self {
        QrEncodeResponse { request_id }
    }

    pub fn get_request_id(&self) -> &str {
        &self.request_id
    }
}

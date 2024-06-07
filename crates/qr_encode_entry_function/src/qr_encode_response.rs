use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct QrEncodeResponse {
    request_id: String,
}

impl QrEncodeResponse {
    pub fn new(request_id: String) -> Self {
        QrEncodeResponse { request_id }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QrResultMonitorRequest {
    request_id: String,
}

impl QrResultMonitorRequest {
    pub fn new(request_id: String) -> Self {
        QrResultMonitorRequest { request_id }
    }

    pub fn get_request_id(&self) -> &str {
        &self.request_id
    }
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct QrResultMonitorRequest {
    request_id: String,
}

impl QrResultMonitorRequest {
    pub fn get_request_id(&self) -> &str {
        &self.request_id
    }
}

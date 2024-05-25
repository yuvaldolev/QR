use serde::Serialize;

#[derive(Serialize)]
pub struct QrEncodeResponse {
    request_id: u128,
}

impl QrEncodeResponse {
    pub fn new(request_id: u128) -> Self {
        QrEncodeResponse { request_id }
    }
}

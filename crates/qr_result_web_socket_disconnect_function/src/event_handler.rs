use lambda_runtime::{tracing, Context};
use qr_aws::functions::web_socket_handler::HandleEvent;

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl HandleEvent for EventHandler {
    type Request = ();
    type Response = String;

    async fn handle_event(
        &self,
        _context: &Context,
        _connection_id: &str,
        request: Option<&Self::Request>,
    ) -> qr_error::Result<Self::Response> {
        tracing::info!("Handling disconnect event");

        request.map_or(Ok(()), |_| Err(qr_error::Error::UnexpectedRequest))?;

        Ok(String::from("Disconnected!"))
    }
}

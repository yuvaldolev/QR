use lambda_runtime::Context;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait::async_trait]
pub trait HandleEvent {
    type Request: DeserializeOwned;
    type Response: Serialize;
    type QueueMessage: Serialize;

    async fn handle_event(
        &self,
        context: &Context,
        request: &Self::Request,
    ) -> qr_error::Result<(Self::QueueMessage, Self::Response)>;
}

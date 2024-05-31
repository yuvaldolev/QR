use serde::{de::DeserializeOwned, Serialize};

#[async_trait::async_trait]
pub trait ProcessEvent {
    type Request: DeserializeOwned;
    type Response: Serialize;
    type QueueMessage: Serialize;

    async fn process_event(
        &self,
        request: Self::Request,
    ) -> qr_error::Result<(Self::QueueMessage, Self::Response)>;
}

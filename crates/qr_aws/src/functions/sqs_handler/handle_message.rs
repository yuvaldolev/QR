use lambda_runtime::Context;
use serde::de::DeserializeOwned;

#[async_trait::async_trait]
pub trait HandleMessage {
    type Message: DeserializeOwned;

    async fn handle_message(
        &self,
        context: &Context,
        id: &str,
        message: Self::Message,
    ) -> qr_error::Result<()>;
}

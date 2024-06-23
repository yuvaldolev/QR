use lambda_runtime::Context;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait::async_trait]
pub trait HandleMessage {
    type InputMessage: DeserializeOwned;
    type OutputMessage: Serialize + Send;

    async fn handle_message(
        &self,
        context: &Context,
        input_id: &str,
        input_receipt_handle: &str,
        input_message: Self::InputMessage,
    ) -> qr_error::Result<Self::OutputMessage>;
}

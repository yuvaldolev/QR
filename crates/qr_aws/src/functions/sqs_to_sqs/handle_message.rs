use lambda_runtime::Context;
use serde::{de::DeserializeOwned, Serialize};

pub trait HandleMessage {
    type InputMessage: DeserializeOwned;
    type OutputMessage: Serialize + Send;

    fn handle_message(
        &self,
        context: &Context,
        input_message_id: &str,
        input_message: Self::InputMessage,
    ) -> Self::OutputMessage;
}

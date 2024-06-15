use super::HandleMessage;

pub trait MakeMessageHandler {
    type MessageHandler: HandleMessage + Send + 'static;

    fn make_message_handler(&self) -> Self::MessageHandler;
}

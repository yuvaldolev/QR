mod handle_message;
mod make_message_handler;

pub use handle_message::HandleMessage;
pub use make_message_handler::MakeMessageHandler;

use aws_lambda_events::sqs::{BatchItemFailure, SqsBatchResponse, SqsEventObj, SqsMessageObj};
use futures::future;
use lambda_runtime::{tracing, Context, LambdaEvent};
use serde_json::Value;
use tokio::task::JoinHandle;

pub struct Function<MessageHandlerFactory> {
    message_handler_factory: MessageHandlerFactory,
}

struct HandleEventArguments<MessageHandler> {
    context: Context,
    message: SqsMessageObj<Value>,
    message_handler: MessageHandler,
}

impl<MessageHandlerFactory> Function<MessageHandlerFactory>
where
    MessageHandlerFactory: MakeMessageHandler,
{
    pub fn new(message_handler_factory: MessageHandlerFactory) -> Self {
        Self {
            message_handler_factory,
        }
    }

    pub async fn run(&self, event: LambdaEvent<SqsEventObj<Value>>) -> SqsBatchResponse {
        tracing::info!("Handling SQS event");

        let (sqs_event, context) = event.into_parts();

        let (ids, tasks): (Vec<_>, Vec<_>) = sqs_event
            .records
            .into_iter()
            .map(|message| {
                Self::create_task(HandleEventArguments {
                    context: context.clone(),
                    message,
                    message_handler: self.message_handler_factory.make_message_handler(),
                })
            })
            .unzip();
        let results = future::join_all(tasks).await;

        let item_failures: Vec<_> = ids
            .into_iter()
            .zip(results.into_iter())
            .filter_map(|(id, result)| {
                let item_result = match result {
                    Ok(item_result) => item_result,
                    Err(e) => {
                        tracing::error!(
                            "Failed to join task '{}': {:#}",
                            id,
                            anyhow::Error::from(e)
                        );
                        return Some(id);
                    }
                };

                match item_result {
                    Ok(_) => None,
                    Err(e) => {
                        tracing::error!(
                            "Failed to handle SQS message: '{}': {:#}",
                            id,
                            anyhow::Error::from(e)
                        );
                        Some(id)
                    }
                }
            })
            .map(|id| BatchItemFailure {
                item_identifier: id,
            })
            .collect();

        SqsBatchResponse {
            batch_item_failures: item_failures,
        }
    }

    fn create_task(
        arguments: HandleEventArguments<MessageHandlerFactory::MessageHandler>,
    ) -> (String, JoinHandle<qr_error::Result<()>>) {
        let id = arguments.message.message_id.clone().unwrap_or_default();
        let task = tokio::spawn(async move { Self::handle_message(arguments).await });

        (id, task)
    }

    async fn handle_message(
        arguments: HandleEventArguments<MessageHandlerFactory::MessageHandler>,
    ) -> qr_error::Result<()> {
        let message_id = arguments.message.message_id.unwrap_or_default();
        tracing::info!("Handling SQS message: '{}'", message_id);

        tracing::trace!(
            "Deserializing message body '{}' from JSON",
            arguments.message.body
        );
        let message: <MessageHandlerFactory::MessageHandler as HandleMessage>::Message =
            serde_json::from_value(arguments.message.body.clone()).map_err(|e| {
                qr_error::Error::DeserializeSqsInputMessage(e, arguments.message.body)
            })?;

        tracing::trace!("Invoking message handler");
        arguments
            .message_handler
            .handle_message(&arguments.context, &message_id, message)
            .await?;

        Ok(())
    }
}

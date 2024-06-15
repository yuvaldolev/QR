mod handle_message;
mod make_message_handler;

pub use handle_message::HandleMessage;
pub use make_message_handler::MakeMessageHandler;

use aws_config::SdkConfig;
use aws_lambda_events::sqs::{BatchItemFailure, SqsBatchResponse, SqsEventObj, SqsMessageObj};
use futures::future;
use lambda_runtime::{tracing, Context, LambdaEvent};
use serde_json::Value;
use tokio::task::JoinHandle;

pub struct Function<MessageHandlerFactory> {
    output_queue_url: String,
    message_handler_factory: MessageHandlerFactory,
    sqs_client: aws_sdk_sqs::Client,
}

struct HandleEventArguments<MessageHandler> {
    output_queue_url: String,
    sqs_client: aws_sdk_sqs::Client,
    context: Context,
    input_message: SqsMessageObj<Value>,
    message_handler: MessageHandler,
}

impl<MessageHandlerFactory> Function<MessageHandlerFactory>
where
    MessageHandlerFactory: MakeMessageHandler,
{
    pub fn new(
        aws_configuration: &SdkConfig,
        output_queue_url: String,
        message_handler_factory: MessageHandlerFactory,
    ) -> Self {
        Self {
            output_queue_url,
            message_handler_factory,
            sqs_client: aws_sdk_sqs::Client::new(&aws_configuration),
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
                    output_queue_url: self.output_queue_url.clone(),
                    sqs_client: self.sqs_client.clone(),
                    context: context.clone(),
                    input_message: message,
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
        let id = arguments
            .input_message
            .message_id
            .clone()
            .unwrap_or_default();
        let task = tokio::spawn(async move { Self::handle_message(arguments).await });

        (id, task)
    }

    async fn handle_message(
        arguments: HandleEventArguments<MessageHandlerFactory::MessageHandler>,
    ) -> qr_error::Result<()> {
        let input_message_id = arguments.input_message.message_id.unwrap_or_default();
        tracing::info!("Handling SQS message: '{}'", input_message_id);

        tracing::trace!(
            "Deserializing input message body '{}' from JSON",
            arguments.input_message.body
        );
        let input_message: <MessageHandlerFactory::MessageHandler as HandleMessage>::InputMessage =
            serde_json::from_value(arguments.input_message.body.clone()).map_err(|e| {
                qr_error::Error::DeserializeSqsInputMessage(e, arguments.input_message.body)
            })?;

        tracing::trace!("Invoking message handler");
        let output_message = arguments
            .message_handler
            .handle_message(&arguments.context, &input_message_id, input_message)
            .await?;

        let output_message_json = serde_json::to_string(&output_message)
            .map_err(qr_error::Error::SerializeSqsOutputMessage)?;
        tracing::trace!(
            "Serialized output message to JSON: '{}'",
            output_message_json
        );

        tracing::info!(
            "Writing message '{}' to SQS '{}",
            output_message_json,
            arguments.output_queue_url
        );
        arguments
            .sqs_client
            .send_message()
            .queue_url(&arguments.output_queue_url)
            .message_body(&output_message_json)
            .send()
            .await
            .map_err(|e| {
                qr_error::Error::SendSqsMessage(
                    e,
                    output_message_json,
                    arguments.output_queue_url.clone(),
                )
            })?;
        tracing::info!("Successfully written message to SQS");

        Ok(())
    }
}

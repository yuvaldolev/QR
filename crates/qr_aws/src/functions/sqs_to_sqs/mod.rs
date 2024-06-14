use std::future::Future;

use aws_config::SdkConfig;
use aws_lambda_events::sqs::{BatchItemFailure, SqsBatchResponse, SqsEventObj, SqsMessageObj};
use futures::future;
use lambda_runtime::{tracing, LambdaEvent};
use serde_json::Value;
use tokio::task::JoinHandle;

pub struct Function<MessageHandler> {
    output_queue_url: String,
    message_handler: MessageHandler,
    sqs_client: aws_sdk_sqs::Client,
}

struct HandleEventArguments<MessageHandler> {
    output_queue_url: String,
    message_handler: MessageHandler,
    sqs_client: aws_sdk_sqs::Client,
    message: SqsMessageObj<Value>,
}

impl<MessageHandler> Function<MessageHandler>
where
    MessageHandler: Clone + Send + 'static,
{
    pub fn new(
        aws_configuration: &SdkConfig,
        output_queue_url: String,
        message_handler: MessageHandler,
    ) -> Self {
        Self {
            output_queue_url,
            message_handler,
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
                    message_handler: self.message_handler.clone(),
                    sqs_client: self.sqs_client.clone(),
                    message,
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

        for failed_item in item_failures.iter() {
            tracing::error!(
                "Failed to handle SQS message: '{}'",
                failed_item.item_identifier
            );
        }

        SqsBatchResponse {
            batch_item_failures: item_failures,
        }
    }

    fn create_task(
        arguments: HandleEventArguments<MessageHandler>,
    ) -> (String, JoinHandle<qr_error::Result<()>>) {
        let id = arguments.message.message_id.clone().unwrap_or_default();
        let task = tokio::spawn(async move { Self::handle_message(arguments).await });

        (id, task)
    }

    async fn handle_message(
        arguments: HandleEventArguments<MessageHandler>,
    ) -> qr_error::Result<()> {
        let SqsMessageObj {
            message_id, body, ..
        } = arguments.message;
        Ok(())
    }
}

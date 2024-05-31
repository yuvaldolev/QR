pub struct SqsToSqsFunction {
    sqs_client: aws_sdk_sqs::Client,
    queue_url: String,
}

// impl SqsToSqsFunction {
//     pub fn new(
// }

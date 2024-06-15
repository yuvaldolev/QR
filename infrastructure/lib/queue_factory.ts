import { Duration, Stack } from "aws-cdk-lib";
import { Queue } from "aws-cdk-lib/aws-sqs";

export class QueueFactory {
  private readonly stack: Stack;

  constructor(stack: Stack) {
    this.stack = stack;
  }

  make(name: string, visibilityTimeout: Duration): Queue {
    const queueName = `${this.stack.stackName}-${name}`;

    return new Queue(this.stack, queueName, {
      queueName,
      visibilityTimeout,
    });
  }
}

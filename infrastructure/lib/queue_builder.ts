import { Duration, Stack } from "aws-cdk-lib";
import { Queue } from "aws-cdk-lib/aws-sqs";

export class QueueBuilder {
  private readonly stack: Stack;
  private readonly name: string;
  private _retentionPeriod?: Duration = undefined;
  private _visibilityTimeout?: Duration = undefined;

  constructor(stack: Stack, name: string) {
    this.stack = stack;
    this.name = `${this.stack.stackName}-${name}`;
  }

  build(): Queue {
    return new Queue(this.stack, this.name, {
      queueName: this.name,
      retentionPeriod: this._retentionPeriod,
      visibilityTimeout: this._visibilityTimeout,
    });
  }

  retentionPeriod(value: Duration): QueueBuilder {
    this._retentionPeriod = value;
    return this;
  }

  visibilityTimeout(value: Duration): QueueBuilder {
    this._visibilityTimeout = value;
    return this;
  }
}

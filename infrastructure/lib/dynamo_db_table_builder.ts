import { RemovalPolicy, Stack } from "aws-cdk-lib";
import {
  Attribute,
  GlobalSecondaryIndexPropsV2,
  ProjectionType,
  TableV2,
} from "aws-cdk-lib/aws-dynamodb";

export class DynamoDbTableBuilder {
  private readonly stack: Stack;
  private readonly name: string;
  private readonly partitionKey: Attribute;
  private readonly removalPolicy: RemovalPolicy;
  private readonly globalSecondaryIndexes: GlobalSecondaryIndexPropsV2[] = [];

  constructor(
    stack: Stack,
    name: string,
    partitionKey: Attribute,
    removalPolicy: RemovalPolicy,
  ) {
    this.stack = stack;
    this.name = `${this.stack.stackName}-${name}`;
    this.partitionKey = partitionKey;
    this.removalPolicy = removalPolicy;
  }

  build(): TableV2 {
    return new TableV2(this.stack, this.name, {
      tableName: this.name,
      partitionKey: this.partitionKey,
      removalPolicy: this.removalPolicy,
      globalSecondaryIndexes: this.globalSecondaryIndexes,
    });
  }

  globalSecondaryIndex(
    name: string,
    key: Attribute,
    projectionType: ProjectionType,
    nonKeyAttributes?: string[],
  ): DynamoDbTableBuilder {
    this.globalSecondaryIndexes.push({
      indexName: name,
      partitionKey: key,
      projectionType: projectionType,
      nonKeyAttributes: nonKeyAttributes,
    });

    return this;
  }
}

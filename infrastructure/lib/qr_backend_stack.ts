import {
  CfnOutput,
  Duration,
  RemovalPolicy,
  Stack,
  StackProps,
} from "aws-cdk-lib";
import { WebSocketApi } from "aws-cdk-lib/aws-apigatewayv2";
import {
  AttributeType,
  ProjectionType,
  TableV2,
} from "aws-cdk-lib/aws-dynamodb";
import { Queue } from "aws-cdk-lib/aws-sqs";
import { Construct } from "constructs";

import { Environment } from "./environment";
import { FunctionEnvironmentBuilder } from "./function_environment_builder";
import { LambdaRestApiBuilder } from "./lambda_rest_api_builder";
import { RustFunctionFactory } from "./rust_function_factory";
import { WebSocketApiBuilder } from "./web_socket_api_builder";
import { DynamoDbTableBuilder } from "./dynamo_db_table_builder";
import { SqsEventSource } from "aws-cdk-lib/aws-lambda-event-sources";
import { QueueFactory } from "./queue_factory";

const ENCODE_RESOURCE = "encode";
const MONITOR_ROUTE = "monitor";

export class QrBackendStack extends Stack {
  constructor(
    scope: Construct,
    id: string,
    environment: Environment,
    props?: StackProps,
  ) {
    super(scope, id, props);

    const rustFunctionFactory = new RustFunctionFactory(this);
    const queueFactory = new QueueFactory(this);

    const encodeEntryQueue = queueFactory.make(
      "QrEncodeEntryQueue",
      Duration.seconds(1),
    );

    const encodeEntryFunction = rustFunctionFactory.make(
      "QrEncodeEntryFunction",
      "qr_encode_entry_function",
      new FunctionEnvironmentBuilder(environment)
        .set("QUEUE_URL", encodeEntryQueue.queueUrl)
        .build(),
    );
    encodeEntryQueue.grantSendMessages(encodeEntryFunction);

    const encodeEntryRestApi = new LambdaRestApiBuilder(
      this,
      "QrEncodeEntryRestApi",
      encodeEntryFunction,
      environment,
    )
      .resource(ENCODE_RESOURCE, "POST")
      .build();

    const resultWebSocketTable = new DynamoDbTableBuilder(
      this,
      "QrResultWebSocketTable",
      {
        name: "requestId",
        type: AttributeType.STRING,
      },
      RemovalPolicy.DESTROY,
    )
      .globalSecondaryIndex(
        "WebSocketIdIndex",
        {
          name: "webSocketId",
          type: AttributeType.STRING,
        },
        ProjectionType.ALL,
      )
      .build();

    const resultWebSocketApi = this.makeResultWebSocket(
      environment,
      rustFunctionFactory,
      resultWebSocketTable,
    );

    const encodeResultFunction = rustFunctionFactory.make(
      "QrEncodeResultFunction",
      "qr_encode_result_function",
      new FunctionEnvironmentBuilder(environment)
        .set("TABLE_NAME", resultWebSocketTable.tableName)
        .set(
          "WEB_SOCKET_API_ENDPOINT",
          `https://${resultWebSocketApi.apiId}.execute-api.${resultWebSocketApi.env.region}.amazonaws.com/${environment}`,
        )
        .build(),
    );
    encodeEntryQueue.grantConsumeMessages(encodeResultFunction);
    encodeResultFunction.addEventSource(
      new SqsEventSource(encodeEntryQueue, {
        batchSize: 10,
        reportBatchItemFailures: true,
      }),
    );
    resultWebSocketTable.grantReadData(encodeResultFunction);
    resultWebSocketApi.grantManageConnections(encodeResultFunction);

    new CfnOutput(this, "encodeApiUrl", {
      value: `${encodeEntryRestApi.url}${ENCODE_RESOURCE}`,
    });

    new CfnOutput(this, "resultWebSocketApiUrl", {
      value: `${resultWebSocketApi.apiEndpoint}/${environment}`,
    });
  }

  private makeResultWebSocket(
    environment: Environment,
    rustFunctionFactory: RustFunctionFactory,
    resultWebSocketTable: TableV2,
  ): WebSocketApi {
    const connectFunction = rustFunctionFactory.make(
      "QrResultWebSocketConnectFunction",
      "qr_result_web_socket_connect_function",
      new FunctionEnvironmentBuilder(environment).build(),
    );

    const disconnectFunction = rustFunctionFactory.make(
      "QrResultWebSocketDisconnectFunction",
      "qr_result_web_socket_disconnect_function",
      new FunctionEnvironmentBuilder(environment)
        .set("TABLE_NAME", resultWebSocketTable.tableName)
        .build(),
    );
    resultWebSocketTable.grantReadWriteData(disconnectFunction);

    const monitorFunction = rustFunctionFactory.make(
      "QrResultWebSocketMonitorFunction",
      "qr_result_web_socket_monitor_function",
      new FunctionEnvironmentBuilder(environment)
        .set("TABLE_NAME", resultWebSocketTable.tableName)
        .build(),
    );
    resultWebSocketTable.grantWriteData(monitorFunction);

    return new WebSocketApiBuilder(
      this,
      "QrResultWebSocketApi",
      environment,
      connectFunction,
      disconnectFunction,
    )
      .route(MONITOR_ROUTE, monitorFunction)
      .build();
  }
}

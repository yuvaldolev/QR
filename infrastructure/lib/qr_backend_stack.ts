import { CfnOutput, Duration, Stack, StackProps } from "aws-cdk-lib";
import { Queue } from "aws-cdk-lib/aws-sqs";
import { Construct } from "constructs";

import { Environment } from "./environment";
import { FunctionEnvironmentBuilder } from "./function_environment_builder";
import { LambdaRestApiBuilder } from "./lambda_rest_api_builder";
import { RustFunctionFactory } from "./rust_function_factory";
import { WebSocketApi } from "aws-cdk-lib/aws-apigatewayv2";
import { WebSocketApiBuilder } from "./web_socket_api_builder";

const ENCODE_RESOURCE = "encode";

export class QrBackendStack extends Stack {
  constructor(
    scope: Construct,
    id: string,
    environment: Environment,
    props?: StackProps,
  ) {
    super(scope, id, props);

    const encodeEntryQueue = new Queue(this, "QrEncodeEntryQueue", {
      visibilityTimeout: Duration.seconds(300),
    });

    const rustFunctionFactory = new RustFunctionFactory(this);

    const encodeEntryFunction = rustFunctionFactory.make(
      "QrEncodeEntryFunction",
      "qr_encode_entry_function",
      new FunctionEnvironmentBuilder(environment)
        .set("QUEUE_URL", encodeEntryQueue.queueUrl)
        .build(),
    );

    const encodeEntryRestApi = new LambdaRestApiBuilder(
      this,
      "QrEncodeEntryRestApi",
      encodeEntryFunction,
      environment,
    )
      .resource(ENCODE_RESOURCE, "POST")
      .build();

    encodeEntryQueue.grantSendMessages(encodeEntryFunction);

    const resultWebSocketApi = this.makeResultWebSocket(
      environment,
      rustFunctionFactory,
    );

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
  ): WebSocketApi {
    const connectFunction = rustFunctionFactory.make(
      "QrResultWebSocketConnectFunction",
      "qr_result_web_socket_connect_function",
      new FunctionEnvironmentBuilder(environment).build(),
    );

    const disconnectFunction = rustFunctionFactory.make(
      "QrResultWebSocketDisconnectFunction",
      "qr_result_web_socket_disconnect_function",
      new FunctionEnvironmentBuilder(environment).build(),
    );

    return new WebSocketApiBuilder(
      this,
      "QrResultWebSocketApi",
      environment,
      connectFunction,
      disconnectFunction,
    ).build();
  }
}

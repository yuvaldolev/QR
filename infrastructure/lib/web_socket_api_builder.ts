import { Stack } from "aws-cdk-lib";
import {
  WebSocketApi,
  WebSocketRouteOptions,
  WebSocketStage,
} from "aws-cdk-lib/aws-apigatewayv2";
import { WebSocketLambdaIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { IFunction } from "aws-cdk-lib/aws-lambda";

import { Environment } from "./environment";

export class WebSocketApiBuilder {
  private readonly apiName: string;
  private readonly webSocketApi: WebSocketApi;

  constructor(
    stack: Stack,
    name: string,
    environment: Environment,
    connectHandler?: IFunction,
    disconnectHandler?: IFunction,
  ) {
    this.apiName = `${stack.stackName}-${name}`;

    this.webSocketApi = new WebSocketApi(stack, this.apiName, {
      apiName: this.apiName,
      connectRouteOptions: this.makeConnectRouteOptions(connectHandler),
      disconnectRouteOptions:
        this.makeDisconnectRouteOptions(disconnectHandler),
    });

    new WebSocketStage(stack, `${this.apiName}-Stage`, {
      webSocketApi: this.webSocketApi,
      stageName: environment,
      autoDeploy: true,
    });
  }

  build(): WebSocketApi {
    return this.webSocketApi;
  }

  route(key: string, handler: IFunction): WebSocketApiBuilder {
    this.webSocketApi.addRoute(key, {
      integration: this.makeLambdaIntegration(key, handler),
    });

    return this;
  }

  private makeConnectRouteOptions(
    handler?: IFunction,
  ): WebSocketRouteOptions | undefined {
    if (typeof handler === "undefined") {
      return undefined;
    }

    return {
      integration: this.makeLambdaIntegration("connect", handler),
    };
  }

  private makeDisconnectRouteOptions(
    handler?: IFunction,
  ): WebSocketRouteOptions | undefined {
    if (typeof handler === "undefined") {
      return undefined;
    }

    return {
      integration: this.makeLambdaIntegration("disconnect", handler),
    };
  }

  private makeLambdaIntegration(
    routeKey: string,
    handler: IFunction,
  ): WebSocketLambdaIntegration {
    return new WebSocketLambdaIntegration(
      `${this.apiName}-${routeKey}-Integration`,
      handler,
    );
  }
}

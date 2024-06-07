import { Stack } from "aws-cdk-lib";
import { WebSocketApi, WebSocketStage } from "aws-cdk-lib/aws-apigatewayv2";
import { Environment } from "./environment";

export class WebSocketApiBuilder {
  private readonly webSocketApi: WebSocketApi;

  constructor(stack: Stack, name: string, environment: Environment) {
    const apiName = `${stack.stackName}-${name}`;

    this.webSocketApi = new WebSocketApi(stack, apiName, {
      apiName,
    });

    new WebSocketStage(stack, `${apiName}-Stage`, {
        webSocketApi: this.webSocketApi,
        stageName : environment,
        autoDeploy : true,
    });
  }

  build(): WebSocketApi { return this.webSocketApi; }

  resource(value: string, ...methods: string[]): LambdaRestApiBuilder {
    const resource = this.lambdaRestApi.root.addResource(value);
    methods.forEach(method => resource.addMethod(method));

    return this;
  }
}

import { Stack } from "aws-cdk-lib";
import { LambdaRestApi } from "aws-cdk-lib/aws-apigateway";
import { IFunction } from "aws-cdk-lib/aws-lambda";

import { Environment } from "./environment";

export class LambdaRestApiBuilder {
  private readonly lambdaRestApi: LambdaRestApi;

  constructor(
    stack: Stack,
    name: string,
    handler: IFunction,
    environment: Environment,
  ) {
    const apiName = `${stack.stackName}-${name}`;
    this.lambdaRestApi = new LambdaRestApi(stack, apiName, {
      restApiName: apiName,
      handler: handler,
      proxy: false,
      deployOptions: {
        stageName: environment,
      },
    });
  }

  build(): LambdaRestApi {
    return this.lambdaRestApi;
  }

  resource(path: string, ...methods: string[]): LambdaRestApiBuilder {
    const resource = this.lambdaRestApi.root.addResource(path);
    methods.forEach((method) => resource.addMethod(method));

    return this;
  }
}

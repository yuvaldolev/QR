import {CfnOutput, Stack, StackProps} from "aws-cdk-lib";
import {Construct} from "constructs";

import {Environment} from "./environment";
import {FunctionEnvironmentBuilder} from "./function_environment_builder";
import {
  LambdaRestApiBuilder,
} from "./lambda_rest_api_builder";
import {RustFunctionFactory} from "./rust_function_factory";

const ENCODE_RESOURCE = "encode";

export class QrBackendStack extends Stack {
  constructor(scope: Construct, id: string, environment: Environment,
              props?: StackProps) {
    super(scope, id, props);

    // example resource
    // const queue = new sqs.Queue(this, 'InfrastructureQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });

    const rustFunctionFactory = new RustFunctionFactory(this);

    const encodeEntryFunction = rustFunctionFactory.make(
        "QrEncodeEntryFunction", "qr_encode_entry_function",
        new FunctionEnvironmentBuilder(environment).build());

    const encodeEntryRestApi =
        new LambdaRestApiBuilder(this, "QrEncodeEntryRestApi",
                                 encodeEntryFunction, environment)
            .resource(ENCODE_RESOURCE, "POST")
            .build();

    new CfnOutput(this, "encodeApiUrl",
                  {value : `${encodeEntryRestApi.url}${ENCODE_RESOURCE}`});
  }
}

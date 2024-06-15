import { Stack } from "aws-cdk-lib";
import { Architecture, IFunction } from "aws-cdk-lib/aws-lambda";
import { LogGroup } from "aws-cdk-lib/aws-logs";
import { RustFunction } from "cargo-lambda-cdk";
import * as path from "path";

const BASE_DIRECTORY = path.join(__dirname, "..", "..");
const MANIFEST_FILE = path.join(BASE_DIRECTORY, "Cargo.toml");

export class RustFunctionFactory {
  private readonly stack: Stack;

  constructor(stack: Stack) {
    this.stack = stack;
  }

  make(
    name: string,
    binary: string,
    environment: { [key: string]: string },
  ): IFunction {
    const functionName = `${this.stack.stackName}-${name}`;

    const logGroupName = `/aws/lambda/${functionName}`;
    const logGroup = new LogGroup(this.stack, logGroupName, {
      logGroupName,
    });

    return new RustFunction(this.stack, functionName, {
      functionName: functionName,
      manifestPath: MANIFEST_FILE,
      binaryName: binary,
      logGroup: logGroup,
      environment: environment,
      bundling: {
        architecture: Architecture.ARM_64,
      },
    });
  }
}

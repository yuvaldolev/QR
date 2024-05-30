import {Stack} from "aws-cdk-lib";
import {Architecture, IFunction} from "aws-cdk-lib/aws-lambda";
import {RustFunction} from "cargo-lambda-cdk";
import * as path from "path";

const BASE_DIRECTORY = path.join(__dirname, "..", "..");
const MANIFEST_FILE = path.join(BASE_DIRECTORY, "Cargo.toml");

export class RustFunctionFactory {
  private readonly stack: Stack;

  constructor(stack: Stack) { this.stack = stack; }

  make(name: string, binary: string,
       environment: {[key: string]: string}): IFunction {
    const functionName = `${this.stack.stackName}-${name}`;
    return new RustFunction(this.stack, functionName, {
      functionName : functionName,
      manifestPath : MANIFEST_FILE,
      binaryName : binary,
      bundling : {
        architecture : Architecture.ARM_64,
        environment : environment,
      },
    });
  }
}

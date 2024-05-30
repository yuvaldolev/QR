import {Environment} from "./environment";

export class FunctionEnvironmentBuilder {
  private readonly functionEnvironment: {[key: string]: string} = {};

  constructor(environment: Environment) {
    this.setupInitialEnvironment(environment);
  }

  build(): {[key: string]: string} { return this.functionEnvironment; }

  set(key: string, value: string): FunctionEnvironmentBuilder {
    this.functionEnvironment[key] = value;
    return this;
  }

  private setupInitialEnvironment(environment: Environment): void {
    if (Environment.Dev === environment) {
      this.functionEnvironment["RUST_BACKTRACE"] = "1";
      this.functionEnvironment["AWS_LAMBDA_LOG_LEVEL"] = "trace";
    } else {
      this.functionEnvironment["RUST_BACKTRACE"] = "0";
      this.functionEnvironment["AWS_LAMBDA_LOG_LEVEL"] = "info";
    }
  }
}

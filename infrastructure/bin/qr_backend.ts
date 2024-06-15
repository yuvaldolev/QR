#!/usr/bin/env node

import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { Environment, QrBackendStack } from "../lib";

const ENVIRONMENTS = [Environment.Dev, Environment.Stage, Environment.Prod];

const app = new cdk.App();

for (const environment of ENVIRONMENTS) {
  new QrBackendStack(app, `QrBackendStack-${environment}`, environment, {});
}

# QR
QR encoder and decoder written in Rust

## TODO:
- [ ] Create prod and stage environments: see [this page](https://stackoverflow.com/questions/68826108/how-to-deploy-to-different-environments-with-aws-sam) and [this page](https://stackoverflow.com/questions/60889263/manage-stage-and-prod-environments-in-aws-sam-cloudformation-template-along-with) for more details
- [ ] Separate the frontend and backend CI/CD into two different jobs in the same pipeline which depend on each other (frontend depends on backend)
      build backend for linux only and frontend for Linux and macOS. Upload the built frontend for each platform as an Artifact
- [ ] Add a nightly pipeline that runs every night, deploys the latest version of the backend to the prod environment and uploads the frontend for each platform to a GitHub release
      (see [this page](https://stackoverflow.com/questions/70281670/checking-condition-when-step-is-running-on-schedule-in-github-actions) for details how to determine if running from schedule)
- [ ] Deploy to stage environment from push trigger of main branch pipeline
- [ ] Add pipeline for only building (CI only) frontend and backend for other branches (not main)

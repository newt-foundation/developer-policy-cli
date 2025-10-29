# Fetcher Server for WASM proxy (farcaster user data)

This is a simple FastAPI app to wrap Vaults.fyi vaults detail fetch to protect your Vaults.fyi
API key when building a Vaults data integration with Newton Protocol policy.

This API is designed to be deployed on AWS Lambda.

It does not natively run in a local environment.

## Steps

Build docker image and publish to ECS

### Base Setup
Create an env file based on .env.example and set your API keys

### Lambda Upload
Create an ECS resource in aws and note the arn.

Then, log into aws from your terminal and connect docker

    aws configure
    export AWS_ACCESS_KEY_ID="MY_KEY_ID"
    export AWS_SECRET_ACCESS_KEY="+MY_ACCESS_KEY"
    export AWS_SESSION_TOKEN="MY_TOKEN"
    aws ecr get-login-password --region my-reg | docker login --username AWS --password-stdin my-ecs-arn

Then, build, tag, and upload the docker image.

    cd server
    docker build -t server .
    docker tag server:latest my-ecs-arn/my-lambda-res:latest
    docker push my-ecs-arn/my-lambda-res:latest

In aws, create a lambda from this image. Set access to public.
Add an API Gatway (REST) trigger in front of the new lambda. Add a public proxy on the root URL.

Visit https://my-deployed-url.amazonaws.com/my-step/hello

Expect:

    message    "Hello World"

Now you can check https://my-deployed-url.amazonaws.com/my-step/0x[some-farcsater-address]
to view proxied user data.

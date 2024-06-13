extern crate contract;
#[macro_use]
extern crate dotenv_codegen;
mod client;
mod dispatcher;
mod util;

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use client::Client;
use dispatcher::Dispatcher;
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};

async fn handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    println!("This is an event: {:?}", event);
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    let client = Client::new(&client);
    let body = event.payload.body.ok_or("No Body provided")?;
    let payload = serde_json::from_str(&body)?;
    let response = Dispatcher::dispatch(&client, payload).await?;
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html".parse()?);
    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        multi_value_headers: headers.clone(),
        is_base64_encoded: false,
        body: Some(serde_json::to_string(&response)?.into()),
        headers,
    };
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}

extern crate contract;
#[macro_use]
extern crate dotenv_codegen;

mod client;
mod dispatcher;
mod entity;

use std::time::Duration;

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use client::Client;
use contract::{
    payload::RequestPayload,
    request::{LogTemperature, Request},
    Client as ContractClient,
};
use dispatcher::Dispatcher;
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

async fn handler(
    connection: &DatabaseConnection,
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    println!("This is an event: {:?}", event);
    let client = Client::initialize(&connection).await?;
    let temperatures = client.list_temperatures().await?;
    println!("Temperatures {}", serde_json::to_string(&temperatures)?);
    let request = RequestPayload {
        request: Request::ListTemperatures,
        operation: contract::operation::Operation::Query,
    };
    println!("List Temperature: {}", serde_json::to_string(&request)?);
    let request = RequestPayload {
        request: Request::LogTemperature(LogTemperature {
            temperature: 32.1,
            humidity: 50.1,
            host_name: "test host".to_owned(),
        }),
        operation: contract::operation::Operation::Mutation,
    };
    println!("Log Temperature: {}", serde_json::to_string(&request)?);
    let body = event.payload.body.ok_or("No Body provided")?;
    println!("Body: {}", body);
    let payload = serde_json::from_str(&body)?;
    let response = Dispatcher::dispatch(&client, payload).await?;
    println!("Response: {}", serde_json::to_string(&response)?);
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
    let mut opt: ConnectOptions = ConnectOptions::new(dotenv!("DATABASE_URL"));
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    let connection = Database::connect(opt).await?;
    let shared_connection = &connection;
    lambda_runtime::run(service_fn(move |event| async move {
        handler(&shared_connection, event).await
    }))
    .await
}

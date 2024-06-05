extern crate contract;
#[macro_use]
extern crate dotenv_codegen;

mod client;
mod entity;

use std::time::Duration;

use aws_lambda_events::{
    apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse},
    event,
};
use client::Client;
use contract::command::{ListTemperatures, ListTemperaturesQuery};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

async fn handler(
    connection: &DatabaseConnection,
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    println!("This is an event: {:?}", event);
    let client = Client::initialize(&connection).await?;
    let temperatures = client.query(ListTemperatures).await?;
    println!(
        "Temperatures {}",
        serde_json::to_string(&temperatures).unwrap()
    );
    let body = event.payload.body.ok_or("No Body provided")?;
    println!("Body: {}", body);
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html".parse().unwrap());
    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        multi_value_headers: headers.clone(),
        is_base64_encoded: false,
        body: Some("Hello AWS Lambda HTTP request".into()),
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

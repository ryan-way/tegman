use aws_lambda_events::eventbridge::EventBridgeEvent;
use client::Client;
use contract::Client as ContractClient;
use lambda_runtime::{service_fn, Error, LambdaEvent};
mod client;

async fn handler(_: LambdaEvent<EventBridgeEvent>) -> Result<(), Error> {
    let client = Client::new();
    println!("Fetching Temperatures");
    let temperatures = match client.list_temperatures().await {
        Ok(result) => result,
        Err(e) => {
            println!("ERROR: {}", e.to_string());
            Err(e)?
        }
    };
    let serialized = serde_json::to_string(&temperatures).unwrap();
    println!("Temperatures: {}", serialized);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}

use aws_lambda_events::eventbridge::EventBridgeEvent;
use lambda_runtime::{service_fn, Error, LambdaEvent};

async fn handler(event: LambdaEvent<EventBridgeEvent>) -> Result<(), Error> {
    println!("This is an event: {:?}", event);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}

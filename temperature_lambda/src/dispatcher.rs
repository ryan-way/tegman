use contract::prelude::*;
use lambda_runtime::Error;

pub struct Dispatcher;

impl Dispatcher {
    pub async fn dispatch<C>(client: &C, request: RequestPayload) -> Result<Response, Error>
    where
        C: Client<Error>,
    {
        let response = match (request.operation, request.request) {
            (Operation::Query, Request::ListTemperatures) => {
                Response::ListTemperatures(client.list_temperatures().await?)
            }
            (Operation::Mutation, Request::LogTemperature(temperature)) => {
                Response::LogTemperature(client.log_temperature(temperature).await?)
            }
            (_, _) => Err("Unsupported operation")?,
        };

        Ok(response)
    }
}

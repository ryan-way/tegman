use crate::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListTemperatures;
#[derive(Serialize, Deserialize)]
pub struct LogTemperature {
    temperature: f32,
}
#[derive(Serialize, Deserialize)]
pub struct TestCode;

#[derive(Serialize, Deserialize)]
pub struct Temperature {
    pub temperature: f32,
}
#[derive(Serialize, Deserialize)]
pub struct EmptyRequest {}

pub trait ListTemperaturesQuery: QueryContract<ListTemperatures> {}
impl<T> QueryContract<ListTemperatures> for T
where
    T: ListTemperaturesQuery,
{
    type Res = Vec<Temperature>;
}
impl<T> ListTemperaturesQuery for T where T: Client {}

pub trait LogTemperatureMutation: MutationContract<LogTemperature> {}
impl<T> MutationContract<LogTemperature> for T
where
    T: LogTemperatureMutation,
{
    type Res = Temperature;
}
impl<T> LogTemperatureMutation for T where T: Client {}

pub trait TestQuery: MutationContract<LogTemperature> {}
impl<T> QueryContract<TestCode> for T
where
    T: LogTemperatureMutation,
{
    type Res = i32;
}

impl<T> TestQuery for T where T: Client {}

pub struct TestClient {}

impl Client for TestClient {
    fn send(data: &str) -> Result<String> {
        Ok(data.to_owned())
    }

    fn serialize<T: Serialize>(data: &T) -> Result<String> {
        serde_json::to_string(&data).map_err(|error| Error::SerializationError(error.to_string()))
    }

    fn deserialize<T: DeserializeOwned>(data: &str) -> Result<T> {
        serde_json::from_str(&data).map_err(|error| Error::SerializationError(error.to_string()))
    }
}

fn test() {
    let test = TestClient {};
    let response = test.query(ListTemperatures);
    let response = test.query(TestCode);
    let response = test.mutation(LogTemperature { temperature: 32.0 });
}

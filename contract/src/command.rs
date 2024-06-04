use serde::{Deserialize, Serialize};

use crate::{
    prelude::{MutationContract, QueryContract},
    Client,
};

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

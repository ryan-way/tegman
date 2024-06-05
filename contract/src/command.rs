use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::base::{MutationContract, QueryContract};

#[derive(Serialize, Deserialize)]
pub struct ListTemperatures;
#[derive(Serialize, Deserialize)]
pub struct LogTemperature {
    pub temperature: f32,
    pub humidity: f32,
    pub host_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Temperature {
    pub temperature: f32,
    pub humidity: f32,
    pub host_name: String,
    pub date: NaiveDateTime,
}

pub trait ListTemperaturesQuery: QueryContract<ListTemperatures> {
    type Err;
    async fn query(&self, command: ListTemperatures) -> Result<Self::Res, Self::Err>;
}

impl<T> QueryContract<ListTemperatures> for T
where
    T: ListTemperaturesQuery,
{
    type Res = Vec<Temperature>;
}

pub trait LogTemperatureMutation: MutationContract<LogTemperature> {
    type Err;
    async fn mutation(&self, command: LogTemperature) -> Result<Self::Res, Self::Err>;
}

impl<T> MutationContract<LogTemperature> for T
where
    T: LogTemperatureMutation,
{
    type Res = Temperature;
}

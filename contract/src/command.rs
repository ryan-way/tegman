use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::util::Result;
use crate::{
    base::{MutationContract, QueryContract},
    Client,
};

#[derive(Serialize, Deserialize)]
pub struct ListTemperatures;
#[derive(Serialize, Deserialize)]
pub struct LogTemperature {
    pub temperature: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Temperature {
    pub temperature: f32,
    pub humidity: f32,
}

pub trait ListTemperaturesQuery: QueryContract<ListTemperatures> {
    fn query(&self, command: ListTemperatures) -> Result<Self::Res>;
}

impl<T, C> QueryContract<C> for T
where
    T: ListTemperaturesQuery,
    C: Serialize + DeserializeOwned,
{
    type Res = Vec<Temperature>;
}

pub trait LogTemperatureMutation: MutationContract<LogTemperature> {
    fn mutation(&self, command: LogTemperature) -> Result<Self::Res>;
}

impl<T, C> MutationContract<C> for T
where
    T: LogTemperatureMutation,
    C: Serialize + DeserializeOwned,
{
    type Res = Temperature;
}

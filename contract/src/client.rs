use crate::{data::Temperature, request::LogTemperature};

pub trait Client<E> {
    async fn log_temperature(&self, temperature: LogTemperature) -> Result<Temperature, E>;
    async fn list_temperatures(&self) -> Result<Vec<Temperature>, E>;
}

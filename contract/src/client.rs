use crate::{data::Temperature, request::LogTemperature};

pub trait Client<E> {
    fn log_temperature(
        &self,
        temperature: LogTemperature,
    ) -> impl std::future::Future<Output = Result<Temperature, E>> + Send;
    fn list_temperatures(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Temperature>, E>> + Send;
}

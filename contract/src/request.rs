use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogTemperature {
    pub temperature: f32,
    pub humidity: f32,
    pub host_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    ListTemperatures,
    LogTemperature(LogTemperature),
}

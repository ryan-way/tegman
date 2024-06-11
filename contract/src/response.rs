use crate::data::Temperature;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    LogTemperature(Temperature),
    ListTemperatures(Vec<Temperature>),
}

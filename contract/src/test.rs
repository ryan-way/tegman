use chrono::Utc;

use crate::{data::Temperature, prelude::*};

pub struct TestClient {}

impl Client<()> for TestClient {
    async fn list_temperatures(&self) -> Result<Vec<Temperature>, ()> {
        Ok(vec![Temperature {
            temperature: 32.0,
            humidity: 50.1,
            host_name: "test_host".to_owned(),
            date: Utc::now(),
        }])
    }

    async fn log_temperature(&self, temperature: LogTemperature) -> Result<Temperature, ()> {
        Ok(Temperature {
            temperature: temperature.temperature,
            humidity: temperature.humidity,
            host_name: temperature.host_name,
            date: Utc::now(),
        })
    }
}

async fn test() {
    let test = TestClient {};
    let response: Result<Vec<Temperature>, ()> = test.list_temperatures().await;
    let response: Result<Temperature, ()> = test
        .log_temperature(LogTemperature {
            temperature: 32.0,
            host_name: "test host".to_owned(),
            humidity: 50.0,
        })
        .await;
}

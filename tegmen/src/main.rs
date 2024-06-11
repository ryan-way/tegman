extern crate dht22_pi;
mod client;
use client::Client;
use contract::request::LogTemperature;
use contract::Client as Contract;
use dht22_pi::read;

fn convert_to_fahrenheit(celcius: f32) -> f32 {
    celcius * 9.0 / 5.0 + 32.0
}

#[tokio::main]
async fn main() {
    let temperature_reading = match read(2) {
        Ok(reading) => reading,
        Err(e) => {
            println!("ERROR: {:?}", e);
            return;
        }
    };
    let client = Client::new();
    let log_temperature = LogTemperature {
        temperature: convert_to_fahrenheit(temperature_reading.temperature),
        humidity: temperature_reading.humidity,
        host_name: "testing real data".to_owned(),
    };
    println!("Temperature reading: {:?}", log_temperature);

    client.log_temperature(log_temperature).await.unwrap();

    println!("Temperature send to api");
}

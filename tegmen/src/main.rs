extern crate dht22_pi;
mod client;
use aws_sdk_cloudwatch::types::{Dimension, MetricDatum};
use client::Client;
use contract::request::LogTemperature;
use contract::Client as Contract;
use dht22_pi::read;

fn convert_to_fahrenheit(celcius: f32) -> f32 {
    celcius * 9.0 / 5.0 + 32.0
}

#[tokio::main]
async fn main() {
    println!("Reading tmperature...");
    let temperature_reading = match read(2) {
        Ok(reading) => reading,
        Err(e) => {
            println!("ERROR: {:?}", e);
            return;
        }
    };
    println!("Creating DTO...");
    let log_temperature = LogTemperature {
        temperature: convert_to_fahrenheit(temperature_reading.temperature),
        humidity: temperature_reading.humidity,
        host_name: hostname::get().unwrap().to_str().unwrap().to_owned(),
    };

    println!("Temperature reading: {:?}", log_temperature);

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_cloudwatch::Client::new(&config);
    let datum = MetricDatum::builder()
        .metric_name("Temperature")
        .dimensions(
            Dimension::builder()
                .name("Host Name")
                .value(&log_temperature.host_name)
                .build(),
        )
        .value(log_temperature.temperature as f64)
        .build();

    client
        .put_metric_data()
        .namespace("tegmen")
        .metric_data(datum)
        .send()
        .await
        .unwrap();

    let datum = MetricDatum::builder()
        .metric_name("Humidity")
        .dimensions(
            Dimension::builder()
                .name("Host Name")
                .value(&log_temperature.host_name)
                .build(),
        )
        .value(log_temperature.humidity as f64)
        .build();

    client
        .put_metric_data()
        .namespace("tegmen")
        .metric_data(datum)
        .send()
        .await
        .unwrap();

    println!("Metrics logged");
    let client = Client::new();
    client.log_temperature(log_temperature).await.unwrap();
    println!("Temperature send to api");
}

use aws_sdk_dynamodb::{types::AttributeValue, Client as DynamoClient};
use chrono::Utc;
use contract::prelude::*;
use lambda_runtime::Error;
use uuid::Uuid;

use crate::util::AttributeParser;

pub struct Client<'a> {
    client: &'a DynamoClient,
}

impl<'a> Client<'a> {
    pub fn new(client: &'a DynamoClient) -> Self {
        Self { client }
    }

    async fn get_temperature(&self, host_name: &str) -> Result<Temperature, Error> {
        let request = self
            .client
            .query()
            .table_name("temperature")
            .index_name("Hostname")
            .key_condition_expression("Hostname = :host_name")
            .expression_attribute_values(":host_name", AttributeValue::S(host_name.into()))
            .scan_index_forward(false)
            .limit(1)
            .into_paginator()
            .items();

        let response = request.send().next().await;
        println!("Response: {:?}", response);

        let attributes =
            response.ok_or(format!("Error getting temperature reading: {}", host_name))??;
        AttributeParser::from(&attributes).try_into()
    }
}

impl<'a> contract::Client<Error> for Client<'a> {
    async fn log_temperature(&self, log_temperature: LogTemperature) -> Result<Temperature, Error> {
        let id = Uuid::new_v4().to_string();
        let id = AttributeValue::S(id);
        let temperature = AttributeValue::N(log_temperature.temperature.to_string());
        let humidity = AttributeValue::N(log_temperature.humidity.to_string());
        let host_name = AttributeValue::S(log_temperature.host_name);
        let date = AttributeValue::S(Utc::now().to_string());

        let request = self
            .client
            .put_item()
            .table_name("temperature")
            .item("Id", id.clone())
            .item("Temperature", temperature)
            .item("Humidity", humidity)
            .item("Hostname", host_name.clone())
            .item("Date", date);

        let response = request.send().await.unwrap();
        println!("Response: {:?}", response);

        let request = self
            .client
            .get_item()
            .table_name("temperature")
            .key("Id", id);

        let response = request.send().await.unwrap();
        let attributes = response.item().ok_or("Item not found")?;

        AttributeParser::from(attributes).try_into()
    }

    async fn list_temperatures(&self) -> Result<Vec<Temperature>, Error> {
        let hosts = ["Host", "Test Host"];

        let mut temperatures = vec![];

        for host in hosts {
            temperatures.push(self.get_temperature(host).await?);
        }

        assert_eq!(temperatures.len(), hosts.len());
        Ok(temperatures)
    }
}

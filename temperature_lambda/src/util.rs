use std::{collections::HashMap, str::FromStr};

use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{DateTime, Utc};
use contract::prelude::Temperature;
use lambda_runtime::Error;

pub struct AttributeParser<'a> {
    attributes: &'a HashMap<String, AttributeValue>,
}

impl<'a> AttributeParser<'a> {
    pub fn from(attributes: &'a HashMap<String, AttributeValue>) -> Self {
        AttributeParser { attributes }
    }

    fn get_string_attribute<'b>(&self, attribute_name: &str) -> Result<String, Error>
    where
        'a: 'b,
    {
        Ok(self
            .get_attribute(attribute_name)?
            .as_s()
            .map_err(|_| "Attribute is a not a string")?
            .to_owned())
    }

    fn get_date_attribute(&self, attribute_name: &str) -> Result<DateTime<Utc>, Error> {
        let string_value = self
            .get_attribute(attribute_name)?
            .as_s()
            .map_err(|_| "Attribute is a not a string")?;
        Ok(string_value
            .parse()
            .map_err(|e| format!("Error parsing date: {}, Error: {}", string_value, e))?)
    }

    fn get_number_attribute<T>(&self, attribute_name: &str) -> Result<T, Error>
    where
        T: FromStr,
        <T as FromStr>::Err: 'static + std::error::Error + Sync + Send,
    {
        Ok(self
            .get_attribute(attribute_name)?
            .as_n()
            .map_err(|_| "Attribute is a not a number")?
            .parse::<T>()?)
    }

    fn get_attribute<'b>(&self, attribute_name: &str) -> Result<&'b AttributeValue, Error>
    where
        'a: 'b,
    {
        self.attributes
            .get(attribute_name)
            .ok_or("Attribute not found".into())
    }
}

impl<'a> TryInto<Temperature> for AttributeParser<'a> {
    type Error = Error;
    fn try_into(self) -> Result<Temperature, Self::Error> {
        Ok(Temperature {
            temperature: self.get_number_attribute("Temperature")?,
            humidity: self.get_number_attribute("Humidity")?,
            host_name: self.get_string_attribute("Hostname")?,
            date: self.get_date_attribute("Date")?,
        })
    }
}

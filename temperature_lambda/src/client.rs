use crate::entity::temperature::{self, Entity};
use chrono::{DateTime, NaiveDateTime, Utc};
use contract::prelude::*;
use lambda_runtime::Error;
use sea_orm::{
    sea_query::{Expr, Query},
    ActiveModelTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

pub struct Client<'a> {
    connection: &'a DatabaseConnection,
}

impl<'a> Client<'a> {
    pub async fn initialize(connection: &'a DatabaseConnection) -> Result<Self, Error> {
        Ok(Self { connection })
    }
}

impl<'a> contract::Client<Error> for Client<'a> {
    async fn log_temperature(&self, temperature: LogTemperature) -> Result<Temperature, Error> {
        let model = temperature::ActiveModel {
            temperature: Set(temperature.temperature),
            humidity: Set(temperature.humidity),
            hostname: Set(temperature.host_name),
            date: Set(Utc::now().naive_utc()),
            ..Default::default()
        }
        .insert(self.connection)
        .await?;

        Ok(Temperature {
            temperature: model.temperature,
            humidity: model.humidity,
            host_name: model.hostname,
            date: model.date,
        })
    }
    async fn list_temperatures(&self) -> Result<Vec<Temperature>, Error> {
        let subquery = Query::select()
            .column(temperature::Column::Hostname)
            .expr(Expr::col(temperature::Column::Date).max())
            .from(temperature::Entity)
            .group_by_col(temperature::Column::Hostname)
            .to_owned();
        let filter = Expr::tuple([
            Expr::col(temperature::Column::Hostname).into(),
            Expr::col(temperature::Column::Date).into(),
        ])
        .in_subquery(subquery);
        Ok(Entity::find()
            .filter(Condition::all().add(filter))
            .all(self.connection)
            .await?
            .iter()
            .map(|model| Temperature {
                temperature: model.temperature,
                humidity: model.humidity,
                date: model.date,
                host_name: model.hostname.clone(),
            })
            .collect())
    }
}

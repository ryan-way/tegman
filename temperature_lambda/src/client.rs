use chrono::NaiveDateTime;
use contract::prelude::*;
use lambda_runtime::Error;
use sea_orm::{
    prelude::{DateTimeUtc, TimeDate},
    sea_query::{Expr, Query},
    ActiveModelTrait, Condition, DatabaseConnection, EntityOrSelect, EntityTrait, QueryFilter,
    QuerySelect, Set,
};

use crate::entity::temperature::{self, Entity};

pub struct Client<'a> {
    connection: &'a DatabaseConnection,
}

impl<'a> Client<'a> {
    pub async fn initialize(connection: &'a DatabaseConnection) -> Result<Self, Error> {
        Ok(Self { connection })
    }
}

impl<'a> LogTemperatureMutation for Client<'a> {
    type Err = Error;
    async fn mutation(&self, command: LogTemperature) -> Result<Self::Res, Self::Err> {
        let model = temperature::ActiveModel {
            temperature: Set(command.temperature),
            humidity: Set(command.humidity),
            hostname: Set(command.host_name),
            date: Set(NaiveDateTime::default()),
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
}

impl<'a> ListTemperaturesQuery for Client<'a> {
    type Err = Error;
    async fn query(&self, command: ListTemperatures) -> Result<Self::Res, Self::Err> {
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

impl<'a> contract::Client for Client<'a> {}

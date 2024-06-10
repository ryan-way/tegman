use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Temperature {
    pub temperature: f32,
    pub humidity: f32,
    pub host_name: String,
    pub date: NaiveDateTime,
}

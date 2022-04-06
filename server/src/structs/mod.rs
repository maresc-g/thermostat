use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};
use chrono::{DateTime, NaiveTime, FixedOffset};

#[derive(Deserialize, Serialize, Debug)]
pub struct TemperatureHistoryRequest {
    pub min_date: DateTime<FixedOffset>,
    pub max_date: DateTime<FixedOffset>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HeaterTimeSlot {
    pub pk: Option<i64>,
    pub target_temperature: f64,
    pub day: i32,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteHeaterTimeSlot {
    pub pk: i64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub settings: HashMap<String, String>
}

use serde_derive::{Deserialize, Serialize};
use chrono::{DateTime, NaiveTime, FixedOffset};

#[derive(Deserialize, Serialize, Debug)]
pub struct TemperatureHistoryRequest {
    pub min_date: DateTime<FixedOffset>,
    pub max_date: DateTime<FixedOffset>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HeaterTimeSlot {
    pub target_temperature: f64,
    pub start_day: i32,
    pub start_time: NaiveTime,
    pub end_day: i32,
    pub end_time: NaiveTime
}
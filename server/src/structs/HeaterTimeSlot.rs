use serde_derive::{Deserialize, Serialize};
use chrono::NaiveTime;

#[derive(Deserialize, Serialize, Debug)]
pub struct HeaterTimeSlot {
    pub target_temperature: f64,
    pub start_day: i32,
    pub start_time: NaiveTime,
    pub end_day: i32,
    pub end_time: NaiveTime
}
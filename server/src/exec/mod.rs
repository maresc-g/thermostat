use std::{time};
use chrono::{Datelike, Local};
use tokio::time::{Duration};
use crate::db::temperature;
use super::db;

const TEMPERATURE_CHECK_DT: u128 = 5000;

pub async fn run_main_loop()  {
    let mut last_temp_check = time::UNIX_EPOCH;
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut now;
    let db = db::DbItf::new().await;
    let mut current_temperature;
    interval.tick().await;
    loop {
        now = Local::now();
        println!("Checking if state needs to change");
        current_temperature = 20_f64; // TODO : read from sensor
        if last_temp_check.elapsed().unwrap().as_millis() > TEMPERATURE_CHECK_DT {
            temperature::insert(&db, current_temperature).await;
            last_temp_check = time::SystemTime::now();
        }
        if let Some(current_timeslot) = db::heater_timeslot::get_current_timeslot(&db, &now.naive_local().weekday().num_days_from_monday(), &now.time()).await {
            if current_temperature < current_timeslot.target_temperature {
                // TODO : turn on relay
                println!("Turning heater on curTemp = {}, targetTemp = {}, date = {}-{}, start = {}-{}, end = {}-{}",
                current_temperature, current_timeslot.target_temperature, now.naive_local().weekday().num_days_from_monday(), now.time(),
                current_timeslot.start_day, current_timeslot.start_time, current_timeslot.end_day, current_timeslot.end_time);
            }
        }
        interval.tick().await;
    }
}
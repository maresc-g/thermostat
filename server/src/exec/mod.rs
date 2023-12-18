use std::{time};
use chrono::{Datelike, Local};
use tokio::time::{Duration};
use crate::db::temperature;
use crate::hal::relay::RelayManager;
use crate::hal::thermometer::ThermometerManager;

use super::db;

const TEMPERATURE_CHECK_DT: u128 = 60000;

pub async fn run_main_loop()  {
    let mut relay = RelayManager::new();
    let mut thermometer = ThermometerManager::new();
    let mut last_temp_check = time::UNIX_EPOCH;
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut now;
    let mut db = db::DbItf::new().await;
    let mut current_temperature;
    let mut heater_running = false;
    let mut last_should_heat;
    {
        let t = db.transaction().await;
        last_should_heat = db::setting::get_bool_by_key(&t, &"is_heating").await;
    }
    interval.tick().await;
    loop {
        let t = db.transaction().await;
        let hysteresis = db::setting::get_float_by_key(&t, &"hysteresis").await;
        now = Local::now();
        current_temperature = thermometer.get_temperature();
        println!("Temp = {:?}", current_temperature);
        if last_temp_check.elapsed().unwrap().as_millis() >= TEMPERATURE_CHECK_DT {
            temperature::insert(&t, current_temperature).await;
            last_temp_check = time::SystemTime::now();
        }

        let mut should_heat = false;
        if let Some(current_timeslot) = db::heater_timeslot::get_current_timeslot(&t, &now.naive_local().weekday().num_days_from_monday(), &now.time()).await {
            if (last_should_heat && current_temperature < current_timeslot.target_temperature + hysteresis)
             || (!last_should_heat && current_temperature < current_timeslot.target_temperature - hysteresis) {
                should_heat = true;
                if !last_should_heat {
                    println!("Turning heater on curTemp = {}, targetTemp = {}, hysteresis = {}, date = {}-{}, timeslot = {}-[{}-{}]",
                             current_temperature, current_timeslot.target_temperature, hysteresis, now.naive_local().weekday().num_days_from_monday(), now.time(),
                             current_timeslot.day, current_timeslot.start_time, current_timeslot.end_time);
                }
            }
        }
        if !should_heat {
            let manual_temp: f64 = db::setting::get_float_by_key(&t, &"manual_mode_temperature").await;
            let value = db::setting::get_bool_by_key(&t, &"manual_mode_enabled").await;
            if value && current_temperature < manual_temp {
                println!("Manual start asked");
                should_heat = true;
            }
        }

        if should_heat && !heater_running {
            heater_running = true;
            println!("Starting heater");
            relay.activate();
        } else if !should_heat && heater_running {
            heater_running = false;
            println!("Stopping heater");
            relay.deactivate();
        }
        db::setting::update_by_key(&t, &"is_heating", &should_heat.to_string()).await;
        last_should_heat = should_heat;
        t.commit().await;
        interval.tick().await;
    }
}

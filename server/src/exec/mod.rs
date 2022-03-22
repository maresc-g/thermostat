use std::{time};
use tokio::time::{Duration};
use crate::db::temperature;
use super::db;

const TEMPERATURE_CHECK_DT: u128 = 5000;

pub async fn run_main_loop()  {
    let mut last_temp_check = time::UNIX_EPOCH;
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let db = db::DbItf::new().await;
    interval.tick().await;
    loop {
        println!("Checking if state needs to change");
        if last_temp_check.elapsed().unwrap().as_millis() > TEMPERATURE_CHECK_DT {
            let temperature = 20_f64;
            temperature::insert(&db, temperature).await;
            last_temp_check = time::SystemTime::now();
        }
        interval.tick().await;
    }
}
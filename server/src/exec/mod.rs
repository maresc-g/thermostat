use std::{thread, time};
use crate::db::temperature;
use super::db;

const TEMPERATURE_CHECK_DT: u128 = 5000;

pub async fn run_main_loop()  {
    let mut last_temp_check = time::UNIX_EPOCH;
    let db = db::DbItf::new().await;
    loop {
        if last_temp_check.elapsed().unwrap().as_millis() > TEMPERATURE_CHECK_DT {
            let temperature = 20_f64;
            temperature::insert(&db, temperature).await;
            last_temp_check = time::SystemTime::now();
        }
        println!("Checking if state needs to change");
        let sec = time::Duration::from_secs(1);
        thread::sleep(sec);
    }
}
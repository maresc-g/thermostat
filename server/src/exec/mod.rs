use std::{thread, time};
use super::db;

const TEMPERATURE_CHECK_DT: u128 = 5000;

pub async fn run_main_loop()  {
    let mut last_temp_check = time::SystemTime::now();
    let db = db::DbItf::new().await;
    loop {
        if last_temp_check.elapsed().unwrap().as_millis() > TEMPERATURE_CHECK_DT {
            let temperature = 20_f64;
            db.query("temperature/insert", &[&temperature]).await;
            last_temp_check = time::SystemTime::now();
        }
        println!("Checking if state needs to change");
        let sec = time::Duration::from_secs(1);
        thread::sleep(sec);
    }
}
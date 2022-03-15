use std::{thread, time};

pub async fn run_main_loop()  {
    loop {
        println!("Checking if state needs to change");
        let sec = time::Duration::from_secs(1);
        thread::sleep(sec);
    }
}
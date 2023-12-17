mod api;
mod exec;
mod db;
mod structs;
mod hal;
#[macro_use]
extern crate log;

extern crate i2cdev;
use std::thread;
use std::time::Duration;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const ADA_SLAVE_ADDR: u16 = 0x18;
const ADA_REG_TEMP: u8 = 0x5;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting up");

    let mut dev = LinuxI2CDevice::new("/dev/i2c-1", ADA_SLAVE_ADDR).unwrap();

    // init sequence
    loop {
        let mut buf: [u8; 2] = [0; 2];
        dev.smbus_write_byte(ADA_REG_TEMP).unwrap();
        dev.read(&mut buf).unwrap();
        println!("Reading: {:?}", buf);
        thread::sleep(Duration::from_millis(200));
    }

//     let handle_main_loop = tokio::spawn(async {
//         exec::run_main_loop().await;
//     });
//     let handle_http_server = tokio::spawn(async {
//         api::run_http_server().await;
//     });
//     handle_main_loop.await.unwrap();
//     handle_http_server.await.unwrap();
}
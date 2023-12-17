extern crate i2cdev;
use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;

use super::Thermometer;

const ADA_SLAVE_ADDR: u16 = 0x18;
const ADA_REG_TEMP: u8 = 0x5;

pub struct Ada {
    device: LinuxI2CDevice,
}

impl Ada {
    pub fn new() -> Ada {
        Ada {
            device: LinuxI2CDevice::new("/dev/i2c-1", ADA_SLAVE_ADDR).unwrap(),
        }
    }

    fn convert(&self, buf: &[u8;2]) -> f64 {
        let mut tmp = buf.clone();
        tmp[0] = tmp[0] & 0x1F;
        if tmp[0] & 0x10 == 0x10 {
            tmp[0] = tmp[0] & 0x0F;
            return (tmp[0] as f64 * 16.0 + tmp[1] as f64 / 16.0) - 256.0;
        }
        return tmp[0] as f64 * 16.0 + tmp[1] as f64 / 16.0;
    }
}

impl Thermometer for Ada {
    fn get_temperature(&mut self) -> f64 {
        let mut buf: [u8; 2] = [0; 2];
        self.device.smbus_write_byte(ADA_REG_TEMP).unwrap();
        self.device.read(&mut buf).unwrap();

        let temperature = self.convert(&buf);
        println!("Reading: {:?}", temperature);
        temperature
    }
}

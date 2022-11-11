use super::Relay;
use gpio::GpioOut;

pub struct Omron {
    gpio: gpio::sysfs::SysFsGpioOutput
}

impl Omron {
    pub fn new() -> Omron {
        Omron {
            gpio: gpio::sysfs::SysFsGpioOutput::open(17).unwrap()
        }
    }
}

impl Relay for Omron {
    fn activate(&mut self) {
        println!("Activating omron relay");
        self.gpio.set_value(true).unwrap();
    }

    fn deactivate(&mut self) {
        println!("Deactivating omron relay");
        self.gpio.set_value(false).unwrap();
    }
}
use super::Relay;
use gpio::GpioOut;

pub struct Omron {
    gpio: gpio::sysfs::SysFsGpioOutput
}

impl Omron {
    pub fn new() -> Omron {
        Omron {
            gpio: gpio::sysfs::SysFsGpioOutput::open(0).unwrap()
        }
    }
}

impl Relay for Omron {
    fn activate(&mut self) {
        self.gpio.set_value(true).unwrap();
    }

    fn deactivate(&mut self) {
        self.gpio.set_value(false).unwrap();
    }
}
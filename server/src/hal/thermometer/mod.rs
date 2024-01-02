pub mod ada;
pub mod simu;

pub trait Thermometer {
    fn get_temperature(&mut self) -> f64;
}

pub struct ThermometerManager {
    thermometer: Box<dyn Thermometer>,
}

impl ThermometerManager {
    #[cfg(target_arch = "aarch64")]
    fn new_thermometer_manager() -> ThermometerManager {
        ThermometerManager {
            thermometer: Box::new(ada::Ada::new()),
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    fn new_thermometer_manager() -> ThermometerManager {
        ThermometerManager {
            thermometer: Box::new(simu::Simu::new()),
        }
    }

    pub fn new() -> ThermometerManager {
        let mut thermometer_manager = Self::new_thermometer_manager();
        thermometer_manager
    }

    pub fn get_temperature(&mut self) -> f64 {
        self.thermometer.get_temperature()
    }
}

unsafe impl Send for ThermometerManager {}
unsafe impl Sync for ThermometerManager {}

use super::Thermometer;

pub struct Simu {
}

impl Simu {
    pub fn new() -> Simu {
        Simu {}
    }
}

impl Thermometer for Simu {
    fn get_temperature(&mut self) -> f64 {
        20.0
    }
}
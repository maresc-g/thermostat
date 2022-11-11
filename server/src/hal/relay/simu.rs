use super::Relay;

pub struct Simu {
}

impl Simu {
    pub fn new() -> Simu {
        Simu {}
    }
}

impl Relay for Simu {
    fn activate(&mut self) {
        println!("Activating simu relay");
    }
    fn deactivate(&mut self) {
        println!("Deactivating simu relay");
    }
}
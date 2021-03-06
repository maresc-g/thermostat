mod simu;
mod omron;

pub trait Relay {
    fn activate(&mut self);
    fn deactivate(&mut self);
}

pub struct RelayManager {
    active: bool,
    relay: Box<dyn Relay>
}

impl RelayManager {
    pub fn new() -> RelayManager {
        let mut relay_manager = if cfg!(arm) {
            RelayManager {
                active: false,
                relay: Box::new(omron::Omron::new())
            }
        }
        else {
            RelayManager {
                active: false,
                relay: Box::new(simu::Simu::new())
            }
        };
        relay_manager.deactivate();
        relay_manager
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.relay.activate();
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.relay.deactivate();
    }

    pub fn switch(&mut self) {
        if self.active {
            self.deactivate();
        }
        else {
            self.activate();
        }
    }
}

unsafe impl Send for RelayManager {}
unsafe impl Sync for RelayManager {}
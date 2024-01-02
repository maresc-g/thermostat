mod omron;
mod simu;

pub trait Relay {
    fn activate(&mut self);
    fn deactivate(&mut self);
}

pub struct RelayManager {
    active: bool,
    relay: Box<dyn Relay>,
}

impl RelayManager {
    #[cfg(target_arch = "aarch64")]
    fn new_relay_manager() -> RelayManager {
        RelayManager {
            active: false,
            relay: Box::new(omron::Omron::new()),
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    fn new_relay_manager() -> RelayManager {
        RelayManager {
            active: false,
            relay: Box::new(simu::Simu::new()),
        }
    }

    pub fn new() -> RelayManager {
        let mut relay_manager = Self::new_relay_manager();
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
        println!(
            "Switching relay state, was {}, now {}",
            self.active, !self.active
        );
        if self.active {
            self.deactivate();
        } else {
            self.activate();
        }
    }
}

unsafe impl Send for RelayManager {}
unsafe impl Sync for RelayManager {}

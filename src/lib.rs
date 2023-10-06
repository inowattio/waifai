use crate::error::WFResult;

mod error;

struct Network {
    ssid: String,
}

pub trait Client {
    fn connect(&self, network: &Network) -> WFResult<()>;
    fn disconnect(&self) -> WFResult<()>;
    fn turn_off(&self) -> WFResult<()>;
    fn turn_on(&self) -> WFResult<()>;
    fn scan(&self) -> WFResult<Vec<Network>>;
    fn is_on(&self) -> WFResult<bool>;
}

pub trait Hotspot {
    fn create(&self, network: Network) -> WFResult<()>;
    fn start(&self) -> WFResult<()>;
    fn stop(&self) -> WFResult<()>;
    fn clients(&self) -> WFResult<Vec<String>>;
    fn is_active(&self) -> WFResult<bool>;
}

pub struct WiFi {
    interface: String,
}

impl WiFi {
    fn new(interface: String) -> Self {
        Self {
            interface
        }
    }

    fn interfaces() -> Vec<String> {
        vec!["lol".to_string()]
    }
}

impl Client for WiFi {
    fn connect(&self, network: &Network) -> WFResult<()> {
        todo!()
    }

    fn disconnect(&self) -> WFResult<()> {
        todo!()
    }

    fn turn_off(&self) -> WFResult<()> {
        todo!()
    }

    fn turn_on(&self) -> WFResult<()> {
        todo!()
    }

    fn scan(&self) -> WFResult<Vec<Network>> {
        todo!()
    }

    fn is_on(&self) -> WFResult<bool> {
        todo!()
    }
}

impl Hotspot for WiFi {
    fn create(&self, network: Network) -> WFResult<()> {
        todo!()
    }

    fn start(&self) -> WFResult<()> {
        todo!()
    }

    fn stop(&self) -> WFResult<()> {
        todo!()
    }

    fn clients(&self) -> WFResult<Vec<String>> {
        todo!()
    }

    fn is_active(&self) -> WFResult<bool> {
        todo!()
    }
}


pub trait Client {
    fn connect(&self);
    fn disconnect(&self);
    fn turn_off(&self);
    fn turn_on(&self);
    fn is_on(&self);
}

pub trait Hotspot {
    fn create(&self);
    fn start(&self);
    fn stop(&self);
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
    fn connect(&self) {
        todo!()
    }

    fn disconnect(&self) {
        todo!()
    }

    fn turn_off(&self) {
        todo!()
    }

    fn turn_on(&self) {
        todo!()
    }

    fn is_on(&self) {
        todo!()
    }
}

impl Hotspot for WiFi {
    fn create(&self) {
        todo!()
    }

    fn start(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }
}

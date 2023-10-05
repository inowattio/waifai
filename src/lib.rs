
pub trait Client {
    fn connect(&self);
    fn disconnect(&self);
}

pub trait Hotspot {
    fn create(&self);
    fn start(&self);
    fn stop(&self);
}

pub struct WiFi;

impl Client for WiFi {
    fn connect(&self) {
        todo!()
    }

    fn disconnect(&self) {
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

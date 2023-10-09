use crate::error::WFResult;

pub trait Hotspot {
    fn create(&self, ssid: String, password: Option<String>) -> WFResult<()>;
    fn start(&self) -> WFResult<()>;
    fn stop(&self) -> WFResult<()>;
    // fn clients(&self) -> WFResult<Vec<String>>;
    // fn is_active(&self) -> WFResult<bool>;
}

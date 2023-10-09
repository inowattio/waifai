use crate::error::WFResult;
use crate::network::Network;

pub trait Client {
    fn connect(&self, ssid: String, password: Option<String>) -> WFResult<bool>;
    fn disconnect(&self) -> WFResult<bool>;
    // fn turn_off(&self) -> WFResult<()>;
    // fn turn_on(&self) -> WFResult<()>;
    // fn is_on(&self) -> WFResult<bool>;
    fn scan(&self, force_rescan: bool) -> WFResult<Vec<Network>>;
    fn connected_network(&self) -> WFResult<Option<Network>>;
    fn is_connected(&self) -> WFResult<bool>;
}

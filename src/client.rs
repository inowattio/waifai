use crate::error::WFResult;
use crate::network::Network;

pub trait Client {
    /// Connect to a network, password is optional as you can connect to networks with no passwords.
    /// Passing a password for a unsecured network doesn't yield in errors/interruptions.
    /// Returns wether the connection was made or not.
    fn connect(&self, ssid: &str, password: Option<&str>) -> WFResult<bool>;
    /// Disconnects from the currently connected network, returns wether the device disconnected
    /// or not (this isn't an error, if you weren't connected to a network beforehand, you didn't
    /// disconnect from any.
    fn disconnect(&self) -> WFResult<bool>;
    // fn turn_off(&self) -> WFResult<()>;
    // fn turn_on(&self) -> WFResult<()>;
    // fn is_on(&self) -> WFResult<bool>;
    /// Scan the currently available networks, pass `force_rescan` as true if you want to rescan
    /// or take whats cached, note that this is a blocking operation.
    fn scan(&self, force_rescan: bool) -> WFResult<Vec<Network>>;
    /// Retrieves the currently connected network if any.
    fn connected_network(&self) -> WFResult<Option<Network>>;
    /// Quickly check if the interface is connected to a network or not, note that this is faster
    /// than checking [connected_network](Client::connected_network).
    fn is_connected(&self) -> WFResult<bool>;
}

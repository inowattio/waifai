
/// A discovered network.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Network {
    /// Wether this network is connected right now.
    pub connected: bool,
    pub bssid: String,
    pub ssid: String,
    pub mode: String,
    pub channel: u32,
    pub rate: String,
    pub signal: u32,
    pub security: String,
}


#[derive(Debug)]
pub struct Network {
    pub connected: bool,
    pub bssid: String,
    pub ssid: String,
    pub mode: String,
    pub channel: u32,
    pub rate: String,
    pub signal: u32,
    pub security: String,
}

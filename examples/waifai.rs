use waifai::*;
use waifai::error::WFError;

fn main() -> Result<(), WFError> {
    let wifi = WiFi::new("wlan0".to_string());

    let networks = wifi.scan(true)?; // the argument forces a rescan or loading from cache
    let my_favorite_network = networks.first().unwrap();
    let connected = wifi.connect(&my_favorite_network.ssid, Some("my_password"))?;

    if connected {
        wifi.disconnect()?;
    } else {
        wifi.create(&my_favorite_network.ssid, Some("password"))?;
        wifi.start()?;
    }

    Ok(())
}

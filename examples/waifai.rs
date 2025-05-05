use waifai::error::WFError;
use waifai::*;

fn main() -> Result<(), WFError> {
    let interfaces = WiFi::interfaces()?;
    let my_interface = interfaces.first().unwrap().clone();
    let wifi = WiFi::new(my_interface);

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

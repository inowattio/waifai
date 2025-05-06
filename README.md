# waifai [![Latest Version](https://img.shields.io/crates/v/waifai.svg?color=yellow)](https://crates.io/crates/waifai) [![Crates.io](https://img.shields.io/crates/d/waifai?color=purple)](https://crates.io/crates/waifai)
**wai·fai** is the pronunciation of the Wi-Fi word.  
This is a library to interact with everything related to Wi-Fi: scan, connect, disconnect and even create hotspots.

**Warning**: This project goes through frequent API breaking changes and hasn't been thoroughly tested.

Currently, only Linux is supported (via `nmcli`), but there are plans to also add Windows support.

## Usage
Minimum Supported Rust Version is `1.61.0`.
```rust
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
```
Check out this at [examples/waifai.rs](examples/waifai.rs).

## Documentation
The documentation is available at [docs.rs](https://docs.rs/waifai/latest/waifai/).  
Curious about the history and what changed between versions? Everything is in the [CHANGELOG](CHANGELOG.md) file.

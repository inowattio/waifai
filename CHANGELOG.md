
# 0.X.Y - DD/MM/YYYY
Nothing yet...

# 0.3.3 - 06/05/2025
- Fix: Wifi::disconnect acts on connections instead of interface
- Fix: In connection name command usages, guard in ticks if it contains spacings
- Fix: Wifi::set_metric would use connection name instead of ssid
- Feat: Add Wifi::auto_connect

# 0.3.2 - 06/05/2025
- Fix: Wifi::up

# 0.3.1 - 06/05/2025
- Feat: Add more debug logs.
- Feat: Add Wifi::up

# 0.3.0 - 06/05/2025
### Changes:
- Feat: `logging` feature flag, enables log facade implementation and logs
- Fix: Revert "Fix: Guard connection-name using commands with ticks"

# 0.2.0 - 30/04/2025
### Changes:
- Docs: fix misspellings
- Feat: Disconnect by connection name
- Fix: Guard connection-name using commands with ticks
- Feat: Metrics get and set
- Feat: Interface device typ

# 0.1.6 - 02/09/2024
### Changes:
- Added `interface` method to get the current interface of an instance.

# 0.1.5 - 30/08/2024
### Changes:
- Added `all_interfaces` method to get devices other than just type `wifi`.

# 0.1.4 - 12/08/2024
### Changes:
- Fixed unhandled `.unwrap()`.
- Fixed ifname wlan0 always being used in `disconnect`.
- Added interfaces fetching

# 0.1.3 - 08/12/2023
### Changes:
- Implemented `std::Error` on `WFError`.
- Fixed `repository` crate link.

# 0.1.2 - 06/11/2023
### Changes:
- Added `is_active` on Hotspot to check if the interface currently runs the hotspot.

# 0.1.1 - 31/10/2023
### Changes:
- Added the `serde` feature flag, it is disabled by default, enables serde Serialize/Deserialize on used types.

### Breaking:
- Replace `String` parameters with `&str`.

# 0.1.0 - 09/10/2023
The first usable version of the crate, yay!
### Changes:
* Linux support using `nmcli`.

# 0.0.0 - 05/10/2023
The first *markdown*, the crate is unusable as it doesn't contain anything helpful.

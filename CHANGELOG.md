
# 0.X.Y - DD/MM/YYYY
### Changes:
Nothing, yet...

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

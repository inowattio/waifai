use crate::client::Client;
use crate::error::WFError::{CommandErr, CommandIO, CommandParse, HotspotCreate, WifiAction};
use crate::error::{WFError, WFResult};
use crate::hotspot::Hotspot;
use crate::log::debug;
use crate::network::Network;
use std::cmp::PartialEq;
use std::process::Command;

fn connection_name_as_param(name: String) -> String {
    if name.contains(" ") {
        format!("'{name}'")
    } else {
        name
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum DeviceType {
    Wifi,
    Ethernet,
    Other(String),
}

impl DeviceType {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let name = name.into().to_ascii_lowercase();
        match name.as_ref() {
            "wifi" => Self::Wifi,
            "ethernet" => Self::Ethernet,
            _ => Self::Other(name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WiFi {
    interface: String,
}

pub fn command(program: &str, args: &[&str]) -> WFResult<String> {
    debug!("Command: {program} {}", args.join(" "));
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|_| CommandIO)?;

    let err: String = String::from_utf8_lossy(&output.stderr)
        .parse()
        .map_err(|_| CommandParse)?;
    debug!("Err: {err}");

    if !err.is_empty() {
        Err(CommandErr(err))?
    }

    let string: String = String::from_utf8_lossy(&output.stdout)
        .parse()
        .map_err(|_| WFError::CommandParse)?;
    debug!("Out: {string}");

    Ok(string.trim().to_string())
}

fn make_table<const N: usize>(data: String) -> WFResult<Vec<[String; N]>> {
    let output = data.lines().collect::<Vec<&str>>();
    let mut output = output.iter();

    let mut words: Vec<String> = Vec::new();
    let mut current_word = String::new();
    let mut in_whitespace = false;
    let header = output.next().ok_or(CommandParse)?;

    for c in header.chars() {
        if c.is_whitespace() {
            current_word.push(c);
            in_whitespace = true;
        } else {
            if in_whitespace {
                words.push(current_word.clone());
                current_word.clear();
            }

            current_word.push(c);
            in_whitespace = false;
        }
    }

    // Add the last word if there's one
    if !current_word.is_empty() {
        words.push(current_word.clone());
    }

    let mut table = Vec::new();
    for line in output.by_ref() {
        let mut index = 0;
        let mut row = [(); N].map(|_| String::new());
        for i in 0..(N - 1) {
            row[i] = line[index..index + words[i].len()].trim().to_string();
            index += words[i].len();
        }

        row[N - 1] = line[index..].trim().to_string();

        table.push(row);
    }

    Ok(table)
}

impl WiFi {
    /// Create a new Wi-Fi instance, pass the interface name.
    /// Note: that this is not checked and might break during operations.
    pub fn new(interface: String) -> Self {
        Self { interface }
    }

    pub fn interface(&self) -> &String {
        &self.interface
    }

    pub fn connection(&self) -> WFResult<String> {
        command(
            "nmcli",
            &[
                "-g",
                "GENERAL.CONNECTION",
                "device",
                "show",
                &self.interface,
            ],
        )
    }

    pub fn metric(&self) -> WFResult<i16> {
        let connection = self.connection()?;

        let output = command(
            "nmcli",
            &["-g", "ipv4.route-metric", "connection", "show", &connection_name_as_param(connection)],
        )?;
        output.parse().map_err(|_| WifiAction(output))
    }

    pub fn set_metric(&self, value: i16) -> WFResult<()> {
        let connection = match self.connected_network()? {
            None => Err(WifiAction("No active connection".to_string()))?,
            Some(network) => network.ssid
        };
        let value = &format!("{value}");

        let output = command(
            "nmcli",
            &["connection", "modify", &connection, "ipv4.route-metric", value],
        )?;
        if !output.is_empty() {
            return Err(WifiAction(output));
        }

        Ok(())
    }

    pub fn interfaces() -> WFResult<Vec<String>> {
        let interfaces = Self::all_interfaces()?;
        Ok(interfaces
            .into_iter()
            .filter(|(_, kind)| *kind == DeviceType::Wifi)
            .map(|(device, _)| device)
            .collect())
    }

    pub fn all_interfaces() -> WFResult<Vec<(String, DeviceType)>> {
        let output = command("nmcli", &["dev"])?;
        Ok(make_table::<4>(output)?
            .into_iter()
            .map(|row| (row[0].clone(), DeviceType::new(row[1].clone())))
            .collect())
    }
    
    pub fn up(&self, ssid: &str) -> WFResult<()> {
        let output = command("nmcli", &["connection", "up", ssid])?;

        if !output.contains("successfully activated") {
            Err(WifiAction(output))?
        }

        Ok(())
    }
}

impl Client for WiFi {
    fn connect(&self, ssid: &str, password: Option<&str>) -> WFResult<bool> {
        let password_arg: &str = match password.as_ref() {
            None => "nopassword",
            Some(password) => password,
        };

        let output = command(
            "nmcli",
            &[
                "device",
                "wifi",
                "connect",
                ssid,
                "password",
                password_arg,
                "ifname",
                &self.interface,
            ],
        )?;

        if output.contains("Secrets were required") {
            return Ok(false);
        }

        if !output.contains("successfully activated") {
            Err(WifiAction(output))?
        }

        Ok(true)
    }

    fn disconnect(&self) -> WFResult<bool> {
        let connection = self.connection()?;
        let output = command("nmcli", &["connection", "down", &connection_name_as_param(connection)])?;

        if !output.contains("successfully deactivated") {
            Err(WifiAction(output))?
        }

        Ok(true)
    }

    fn scan(&self, force_rescan: bool) -> WFResult<Vec<Network>> {
        let force_rescan = match force_rescan {
            true => "yes",
            false => "no",
        };

        let output = command(
            "nmcli",
            &[
                "device",
                "wifi",
                "list",
                "ifname",
                &self.interface,
                "--rescan",
                force_rescan,
            ],
        )?;

        Ok(make_table::<8>(output)?
            .into_iter()
            .map(|row| Network {
                connected: row[0].contains("*"),
                bssid: row[1].clone(),
                ssid: row[2].clone(),
                mode: row[3].clone(),
                channel: row[4].parse().unwrap_or(0),
                rate: row[5].clone(),
                signal: row[6].parse().unwrap_or(0),
                security: row[7].clone(),
            })
            .collect())
    }

    fn connected_network(&self) -> WFResult<Option<Network>> {
        if !self.is_connected()? {
            return Ok(None);
        }

        let networks = self.scan(false)?;

        for network in networks {
            if network.connected {
                return Ok(Some(network));
            }
        }

        Ok(None)
    }

    fn is_connected(&self) -> WFResult<bool> {
        let output = command("nmcli", &["--fields", "DEVICE,STATE", "device", "status"])?;
        let entries = make_table::<2>(output)?;

        for [device, status] in entries {
            if device == self.interface {
                return Ok(status == "connected");
            }
        }

        Ok(false)
    }
}

impl Hotspot for WiFi {
    fn create(&self, ssid: &str, password: Option<&str>) -> WFResult<()> {
        let id = "Hotspot"; // TODO: dynamic ids/use uuid
        let _ = command("nmcli", &["con", "delete", id]);

        let output = command(
            "nmcli",
            &[
                "con",
                "add",
                "type",
                "wifi",
                "ifname",
                &self.interface,
                "con-name",
                id,
                "autoconnect",
                "yes",
                "ssid",
                ssid,
            ],
        )?;

        if !output.contains("successfully added") {
            Err(HotspotCreate(output))?
        }

        let output = command(
            "nmcli",
            &[
                "con",
                "modify",
                "Hotspot",
                "802-11-wireless.mode",
                "ap",
                "802-11-wireless.band",
                "bg",
                "ipv4.method",
                "shared",
            ],
        )?;

        if !output.is_empty() {
            Err(HotspotCreate(output))?
        }

        if let Some(password) = password {
            let output = command(
                "nmcli",
                &["con", "modify", "Hotspot", "wifi-sec.key-mgmt", "wpa-psk"],
            )?;

            if !output.is_empty() {
                Err(HotspotCreate(output))?
            }

            let output = command(
                "nmcli",
                &["con", "modify", "Hotspot", "wifi-sec.psk", password],
            )?;

            if !output.is_empty() {
                Err(HotspotCreate(output))?
            }
        }

        Ok(())
    }

    fn start(&self) -> WFResult<()> {
        let output = command("nmcli", &["con", "up", "Hotspot"])?;

        if !output.contains("Connection successfully activated") {
            Err(HotspotCreate(output))?
        }

        Ok(())
    }

    fn stop(&self) -> WFResult<()> {
        let output = command("nmcli", &["con", "down", "Hotspot"])?;

        if !output.contains("successfully deactivated") {
            Err(HotspotCreate(output))?
        }

        Ok(())
    }

    fn is_active(&self) -> WFResult<bool> {
        let output = command("nmcli", &["con", "show", "--active"])?;

        Ok(output.contains("Hotspot"))
    }
}

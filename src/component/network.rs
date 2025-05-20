use std::fmt::Write;
use std::process::Command;

use super::Component;

pub struct NetworkSSID;

impl Component for NetworkSSID {
    fn update(&mut self, buf: &mut String) {
        let ssid = get_active_ssid();
        let ssid = ssid.as_deref().unwrap_or("---");

        buf.clear();
        write!(buf, "{}", ssid).expect("wifi ssid write error");
    }
}

fn get_active_ssid() -> Option<String> {
    // TODO: make this faster or non-blocking
    let output = Command::new("nmcli")
        .args(["-t", "-f", "active,ssid", "dev", "wifi"])
        .output()
        .ok()?;

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| line.strip_prefix("yes:"))
        .map(String::from)
}

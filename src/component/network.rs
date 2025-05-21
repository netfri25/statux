use std::fmt::Write;
use tokio::process::Command;

use super::{Component, EMPTY_OUTPUT};

pub struct NetworkSSID;

impl Component for NetworkSSID {
    async fn update(&mut self, buf: &mut String) {
        let ssid = get_active_ssid().await;
        let ssid = ssid.as_deref().unwrap_or(EMPTY_OUTPUT);

        buf.clear();
        write!(buf, "{}", ssid).expect("wifi ssid write error");
    }
}

async fn get_active_ssid() -> Option<String> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "active,ssid", "dev", "wifi"])
        .output()
        .await
        .ok()?;

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| line.strip_prefix("yes:"))
        .map(String::from)
}

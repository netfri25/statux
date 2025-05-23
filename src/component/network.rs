use std::fmt::Write;
use tokio::process::Command;

use super::{Component, EMPTY_OUTPUT};

pub struct NetworkSSID;

impl Component for NetworkSSID {
    async fn update(&mut self, buf: &mut String) -> anyhow::Result<()> {
        let ssid = get_active_ssid().await;
        let ssid = ssid.as_deref().unwrap_or(EMPTY_OUTPUT);

        write!(buf, "{}", ssid)?;
        Ok(())
    }
}

async fn get_active_ssid() -> anyhow::Result<String> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "active,ssid", "dev", "wifi"])
        .output()
        .await?;

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| line.strip_prefix("yes:"))
        .map(String::from)
        .ok_or_else(|| anyhow::anyhow!("no active ssid"))
}

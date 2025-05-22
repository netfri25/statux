use component::{BatteryLevel, BatteryTimeLeft, CpuUsage, DiskUsage, NetworkSSID, Playing, RamUsed, Time, Volume};
use context::Context;
use std::time::Duration;

mod component;
mod context;

const VOLUME_SIGNAL: u8 = 1;
const PLAYING_SIGNAL: u8 = 2;

macro_rules! label {
    ($name:tt) => {
        // #define YELLOW(x) "^c#888811^" x "^d^"
        concat!("^c#888811^", $name, "^d^")
    }
}

fn main() {
    let body = async {
        Context::new()
            .add_timed_signal(PLAYING_SIGNAL, Duration::from_secs(5), Playing)
            .seperator()
            .add_static(label!("CPU"))
            .add_timed(Duration::from_secs(1), CpuUsage::new())
            .seperator()
            .add_static(label!("DISK"))
            .add_timed(Duration::from_secs(30), DiskUsage::new("/"))
            .seperator()
            .add_static(label!("RAM"))
            .add_timed(Duration::from_secs(5), RamUsed)
            .seperator()
            .add_static(label!("VOL"))
            .add_timed_signal(VOLUME_SIGNAL, Duration::from_secs(10), Volume)
            .seperator()
            .add_static(label!("WIFI"))
            .add_timed(Duration::from_secs(10), NetworkSSID)
            .seperator()
            .add_static(label!("TIME"))
            .add_timed(Duration::from_secs(60), Time::new("%H:%M"))
            .add_timed(Duration::from_secs(60 * 60 * 24), Time::new("%d/%m"))
            .seperator()
            .add_static(label!("LEFT"))
            .add_timed(Duration::from_secs(2), BatteryTimeLeft)
            .seperator()
            .add_static(label!("BAT"))
            .add_timed(Duration::from_secs(2), BatteryLevel)
            .run().await;
    };

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body)
}

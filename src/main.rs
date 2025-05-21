use component::{CpuUsage, DiskUsage, NetworkSSID, RamUsed, Time};
use context::Context;
use std::time::Duration;

mod component;
mod context;

#[tokio::main]
async fn main() {
    Context::new()
        .add_static("CPU")
        .add_timed(Duration::from_secs(1), CpuUsage::new())
        .seperator()
        .add_static("RAM")
        .add_timed(Duration::from_secs_f32(0.5), RamUsed::new())
        .seperator()
        .add_static("TIME")
        .add_timed(Duration::from_secs(5), Time::new("%S"))
        .seperator()
        .add_static("DISK")
        .add_timed(Duration::from_secs(30), DiskUsage::new("/"))
        .seperator()
        .add_static("WIFI")
        .add_timed(Duration::from_secs(3), NetworkSSID)
        .run().await;
}

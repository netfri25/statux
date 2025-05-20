use component::{CpuUsage, DiskUsage, RamUsed, Time};
use executer::Executer;
use std::time::Duration;

mod component;
mod executer;

fn main() {
    Executer::new()
        .add_static("CPU")
        .add_timed(Duration::from_secs(1), CpuUsage::new())
        .add_static(())
        .add_static("RAM")
        .add_timed(Duration::from_secs_f32(0.5), RamUsed::new())
        .add_static(())
        .add_static("TIME")
        .add_timed(Duration::from_secs(5), Time::new("%S"))
        .add_static(())
        .add_static("DISK")
        .add_timed(Duration::from_secs(30), DiskUsage::new("/"))
        .run();
}

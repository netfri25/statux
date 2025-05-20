use component::{CpuUsage, RamUsage, Time};
use executer::Executer;
use std::time::Duration;

mod component;
mod executer;

fn main() {
    Executer::new()
        .add_static("CPU")
        .add_timed(Duration::from_secs(1), CpuUsage::new())
        .add_static(())
        .add_timed(Duration::from_secs_f32(0.5), RamUsage::new())
        .add_static(())
        .add_timed(Duration::from_secs(5), Time::new("%S"))
        .run();
}

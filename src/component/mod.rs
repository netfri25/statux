pub mod time;
pub mod cpu;
pub mod memory;
pub mod disk;
pub mod network;
pub mod battery;
pub mod volume;
pub mod playing;

use std::time::Duration;

pub use time::Time;
pub use cpu::CpuUsage;
pub use memory::RamUsed;
pub use disk::DiskUsage;
pub use network::NetworkSSID;
pub use battery::{BatteryLevel, BatteryTimeLeft};
pub use volume::Volume;
pub use playing::Playing;

pub const EMPTY_OUTPUT: &str = "---";

// NOTE: this exist for the purpose of dealing with system suspend. since tokio::time::sleep
//       doesn't consider system suspension, the minimum time that the sleep function is allowed to
//       sleep is the time defined here, and by that we get a maximum error of 10s, which is reasonable.
pub const MIN_UPDATE_TIME: Duration = Duration::from_secs(10);

pub trait Component {
    fn update(&mut self, buf: &mut String) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl Component for &str {
    async fn update(&mut self, buf: &mut String) -> anyhow::Result<()> {
        buf.push_str(self);
        Ok(())
    }
}

// `do nothing` component. can be used as a seperator
impl Component for () {
    async fn update(&mut self, _: &mut String) -> anyhow::Result<()> {
        Ok(())
    }
}

pub mod battery;
pub mod cpu;
pub mod disk;
pub mod memory;
pub mod network;
pub mod playing;
pub mod time;
pub mod volume;

use std::time::Duration;

pub use battery::{BatteryLevel, BatteryTimeLeft};
pub use cpu::CpuUsage;
pub use disk::DiskUsage;
pub use memory::RamUsed;
pub use network::NetworkSSID;
pub use playing::Playing;
pub use time::Time;
pub use volume::Volume;

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

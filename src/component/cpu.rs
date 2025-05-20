use std::fmt::Write;

use sysinfo::{CpuRefreshKind, RefreshKind, System};

use super::Component;

pub struct CpuUsage {
    system: System,
}

impl CpuUsage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CpuUsage {
    fn default() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::nothing().with_cpu_usage()),
        );
        Self { system }
    }
}

impl Component for CpuUsage {
    fn update(&mut self, buf: &mut String) {
        self.system.refresh_cpu_usage();
        let usage = self.system.global_cpu_usage();

        buf.clear();
        write!(buf, "{:2.0}", usage).expect("cpu usage write error")
    }
}

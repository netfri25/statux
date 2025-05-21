use std::fmt::Write;

use tokio::io::AsyncBufReadExt;

use super::{Component, EMPTY_OUTPUT};

struct CpuTime {
    active: u64,
    idle: u64,
}

#[derive(Default)]
pub struct CpuUsage {
    prev: Option<CpuTime>,
}

impl CpuUsage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for CpuUsage {
    async fn update(&mut self, buf: &mut String) {
        let current = get_cpu_time().await.expect("unable to get cpu times");

        let Some(ref prev) = self.prev else {
            self.prev = Some(current);
            write!(buf, "{}", EMPTY_OUTPUT).expect("cpu usage write error");
            return;
        };

        let current_total = current.idle + current.active;
        let prev_total = prev.idle + prev.active;
        let total_diff = current_total - prev_total;
        let active_diff = current.active - prev.active;
        let usage = 100. * active_diff as f32 / total_diff as f32;

        self.prev = Some(current);
        buf.clear();
        write!(buf, "{:4.1}%", usage).expect("cpu usage write error");
    }
}

async fn get_cpu_time() -> Option<CpuTime> {
    let file = tokio::fs::File::open("/proc/stat").await.ok()?;
    let reader = tokio::io::BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.ok()? {
        let Some(line) = line.strip_prefix("cpu ") else {
            continue
        };

        let mut parts = line.split_whitespace();
        let user: u64 = parts.next()?.parse().ok()?;
        let nice: u64 = parts.next()?.parse().ok()?;
        let system: u64 = parts.next()?.parse().ok()?;
        let idle: u64 = parts.next()?.parse().ok()?;
        let iowait: u64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or_default();

        let active = user + nice + system;
        let idle = idle + iowait;
        return Some(CpuTime { active, idle })
    }

    None
}

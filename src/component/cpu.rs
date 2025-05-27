use std::fmt::Write;

use anyhow::Context;
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
    async fn update(&mut self, buf: &mut String) -> anyhow::Result<()> {
        let current = get_cpu_time().await?;

        let Some(ref prev) = self.prev else {
            self.prev = Some(current);
            write!(buf, " {}", EMPTY_OUTPUT)?;
            return Ok(());
        };

        let current_total = current.idle + current.active;
        let prev_total = prev.idle + prev.active;
        let total_diff = current_total - prev_total;
        let active_diff = current.active - prev.active;
        let usage = 100 * active_diff / total_diff;

        self.prev = Some(current);
        write!(buf, "{:2}", usage)?;
        Ok(())
    }
}

async fn get_cpu_time() -> anyhow::Result<CpuTime> {
    let file = tokio::fs::File::open("/proc/stat").await?;
    let reader = tokio::io::BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let Some(line) = line.strip_prefix("cpu ") else {
            continue;
        };

        let mut parts = line.split_whitespace();
        let user: u64 = parts.next().context("no user cpu-time")?.parse()?;
        let nice: u64 = parts.next().context("no nice cpu-time")?.parse()?;
        let system: u64 = parts.next().context("no system cpu-time")?.parse()?;
        let idle: u64 = parts.next().context("no idle cpu-time")?.parse()?;
        let iowait: u64 = parts
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or_default();

        let active = user + nice + system;
        let idle = idle + iowait;
        return Ok(CpuTime { active, idle });
    }

    anyhow::bail!("unable to get cpu-time")
}

use std::fmt::Write;

use anyhow::Context;
use tokio::io::AsyncBufReadExt;

use super::Component;

pub struct RamUsed;

impl Component for RamUsed {
    async fn update(&mut self, buf: &mut String) -> anyhow::Result<()> {
        let used_bytes = get_used_bytes().await?;

        let (unit_bytes, unit) = max_unit(used_bytes);
        let used = used_bytes as f32 / unit_bytes as f32;

        write!(buf, "{:2.1} {}", used, unit)?;
        Ok(())
    }
}

fn parse_field(line: &str, field: &str) -> anyhow::Result<u64> {
    let output = line
        .strip_prefix(field)
        .with_context(|| format!("incorrect field {}", field))?
        .split_whitespace()
        .next()
        .context("missing value part")?
        .parse()?;
    Ok(output)
}

async fn get_used_bytes() -> anyhow::Result<u64> {
    let file = tokio::fs::File::open("/proc/meminfo").await?;
    let reader = tokio::io::BufReader::new(file);
    let mut lines = reader.lines();

    let line = lines.next_line().await?.context("missing line")?;
    let mem_total = parse_field(&line, "MemTotal:")?;

    let line = lines.next_line().await?.context("missing line")?;
    let mem_free = parse_field(&line, "MemFree:")?;

    lines.next_line().await.ok();

    let line = lines.next_line().await?.context("missing line")?;
    let buffers = parse_field(&line, "Buffers:")?;

    let line = lines.next_line().await?.context("missing line")?;
    let cached = parse_field(&line, "Cached:")?;

    let total_bytes = 1024 * (mem_total - mem_free - buffers - cached);
    Ok(total_bytes)
}

fn max_unit(bytes: u64) -> (u64, &'static str) {
    [(1 << 30, "Gi"), (1 << 20, "Mi"), (1 << 10, "Ki")]
        .into_iter()
        .find(|(bytes_in_unit, _)| bytes >= *bytes_in_unit)
        .unwrap_or((bytes, "B"))
}

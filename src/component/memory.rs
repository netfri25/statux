use std::fmt::Write;

use tokio::io::AsyncBufReadExt;

use super::Component;

pub struct RamUsed;

impl Component for RamUsed {
    async fn update(&mut self, buf: &mut String) {
        let used_bytes = get_used_bytes().await.expect("unable to get used bytes");

        let (unit_bytes, unit) = max_unit(used_bytes);
        let used = used_bytes as f32 / unit_bytes as f32;

        buf.clear();
        write!(buf, "{:2.1} {}", used, unit).expect("ram usage write error")
    }
}

async fn get_used_bytes() -> Option<u64> {
    let file = tokio::fs::File::open("/proc/meminfo").await.ok()?;
    let reader = tokio::io::BufReader::new(file);
    let mut lines = reader.lines();

    let line = lines.next_line().await.ok()??;
    let mem_total: u64 = line.strip_prefix("MemTotal:")?.split_whitespace().next()?.parse().ok()?;

    let line = lines.next_line().await.ok()??;
    let mem_free: u64 = line.strip_prefix("MemFree:")?.split_whitespace().next()?.parse().ok()?;

    lines.next_line().await.ok()??;

    let line = lines.next_line().await.ok()??;
    let buffers: u64 = line.strip_prefix("Buffers:")?.split_whitespace().next()?.parse().ok()?;

    let line = lines.next_line().await.ok()??;
    let cached: u64 = line.strip_prefix("Cached:")?.split_whitespace().next()?.parse().ok()?;

    Some(1024 * (mem_total - mem_free - buffers - cached))
}

fn max_unit(bytes: u64) -> (u64, &'static str) {
    [(1 << 30, "Gi"), (1 << 20, "Mi"), (1 << 10, "Ki")]
        .into_iter()
        .find(|(bytes_in_unit, _)| bytes >= *bytes_in_unit)
        .unwrap_or((bytes, "B"))
}

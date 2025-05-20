use std::fmt::Write;

use sysinfo::{MemoryRefreshKind, RefreshKind, System};

use super::Component;

pub struct RamUsed {
    system: System,
}

impl RamUsed {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for RamUsed {
    fn default() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram()),
        );
        Self { system }
    }
}

impl Component for RamUsed {
    fn update(&mut self, buf: &mut String) {
        self.system
            .refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());

        let used_bytes = self.system.used_memory();
        let (bytes_in_unit, unit) = [(1 << 30, "Gi"), (1 << 20, "Mi"), (1 << 10, "Ki")]
            .into_iter()
            .find(|(bytes_in_unit, _)| used_bytes >= *bytes_in_unit)
            .unwrap_or((used_bytes, "B"));

        let used = used_bytes as f32 / bytes_in_unit as f32;

        buf.clear();
        write!(buf, "{:2.1} {}", used, unit).expect("ram usage write error")
    }
}

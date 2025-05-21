use std::fmt::Write;
use std::path::Path;

use sysinfo::{DiskRefreshKind, Disks};

use super::Component;

pub struct DiskUsage {
    disks: Disks,
    path: Box<Path>,
}

impl DiskUsage {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf().into_boxed_path();
        let disks = Disks::new();
        Self { disks, path }
    }
}

impl Component for DiskUsage {
    async fn update(&mut self, buf: &mut String) {
        self.disks
            .refresh_specifics(true, DiskRefreshKind::nothing().with_storage());

        let Some(disk) = self
            .disks
            .iter()
            .find(|disk| disk.mount_point() == self.path.as_ref())
        else {
            panic!("{} is not mounted", self.path.display());
        };

        let free = disk.available_space() as f32 / disk.total_space() as f32;
        let usage = (1. - free) * 100.0;

        buf.clear();
        write!(buf, "{:2.1}%", usage).expect("disk usage write error");
    }
}

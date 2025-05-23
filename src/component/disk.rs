use std::fmt::Write;
use std::path::Path;

use super::Component;

pub struct DiskUsage {
    path: Box<Path>,
}

impl DiskUsage {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf().into_boxed_path();
        Self { path }
    }
}

impl Component for DiskUsage {
    async fn update(&mut self, buf: &mut String) -> anyhow::Result<()> {
        let stat = nix::sys::statvfs::statvfs(self.path.as_ref())?;

        let usage = 1. - stat.blocks_available() as f32 / stat.blocks() as f32;
        let usage = 100. * usage;

        write!(buf, "{:2.1}%", usage)?;
        Ok(())
    }
}

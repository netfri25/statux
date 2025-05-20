pub mod time;
pub mod cpu;
pub mod memory;
pub mod disk;

pub use time::Time;
pub use cpu::CpuUsage;
pub use memory::RamUsed;
pub use disk::DiskUsage;

pub trait Component {
    fn update(&mut self, buf: &mut String);
}

impl Component for &str {
    fn update(&mut self, buf: &mut String) {
        buf.clear();
        buf.push_str(self)
    }
}

// `do nothing` component. can be used as a seperator
impl Component for () {
    fn update(&mut self, _: &mut String) {}
}

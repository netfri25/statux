use component::{
    BatteryLevel, BatteryTimeLeft, CpuUsage, DiskUsage, NetworkSSID, Playing, RamUsed, Time, Volume,
};
use context::Context;
use schedule::Schedule;

mod component;
mod context;
mod schedule;

const VOLUME_SIGNAL: u8 = 1;
const PLAYING_SIGNAL: u8 = 2;

macro_rules! label {
    ($name:tt) => {
        // #define YELLOW(x) "^c#888811^" x "^d^"
        concat!("^c#888811^", $name, "^d^")
    };
}

fn main() {
    let body = async {
        Context::new()
            .add_timed_signal(PLAYING_SIGNAL, Schedule::every().sec(), Playing)
            .seperator()
            .add_static(label!("CPU"))
            .add_timed(Schedule::every().sec(), CpuUsage::new())
            .seperator()
            .add_static(label!("DISK"))
            .add_timed(Schedule::every().secs(30), DiskUsage::new("/"))
            .seperator()
            .add_static(label!("RAM"))
            .add_timed(Schedule::every().secs(5), RamUsed)
            .seperator()
            .add_static(label!("VOL"))
            .add_timed_signal(VOLUME_SIGNAL, Schedule::every().secs(10), Volume)
            .seperator()
            .add_static(label!("WIFI"))
            .add_timed(Schedule::every().secs(10), NetworkSSID)
            .seperator()
            .add_static(label!("TIME"))
            .add_timed(Schedule::every().min(), Time::new("%H:%M"))
            .add_timed(Schedule::every().sec(), Time::new("%Ss"))
            .add_timed(Schedule::every().day(), Time::new("%d/%m"))
            .seperator()
            .add_static(label!("LEFT"))
            .add_timed(Schedule::every().secs(2), BatteryTimeLeft)
            .seperator()
            .add_static(label!("BAT"))
            .add_timed(Schedule::every().secs(2), BatteryLevel)
            .run()
            .await;
    };

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body)
}

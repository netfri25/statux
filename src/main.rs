use component::Time;
use executer::Executer;
use std::time::Duration;

mod component;
mod executer;

fn main() {
    Executer::new()
        .add_static("every2")
        .add_timed(Duration::from_secs(2), Time::new("%S"))
        .add_static(())
        .add_static("every5")
        .add_timed(Duration::from_secs(5), Time::new("%S"))
        .run();
}

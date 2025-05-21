use std::fmt::Write;

use battery::{Manager, Battery, State};

use super::{Component, EMPTY_OUTPUT};

pub struct BatteryLevel;

impl Component for BatteryLevel {
    async fn update(&mut self, buf: &mut String) {
        let state = match battery().state() {
            State::Charging => '+',
            State::Discharging => '-',
            _ => 'o',
        };

        let charge = battery().state_of_charge().value * 100.;
        write!(buf, "{}{}", state, charge.trunc()).expect("battery charge write error");
    }
}

pub struct BatteryTimeLeft;

impl Component for BatteryTimeLeft {
    async fn update(&mut self, buf: &mut String) {
        let battery = battery();
        let state = battery.state();
        let time_left = match state {
            State::Charging => battery.time_to_full(),
            State::Discharging => battery.time_to_empty(),
            _ => None,
        };

        if let Some(time_left) = time_left {
            let hours = time_left.value / 3600.;
            let minutes = hours.fract() * 60.;
            if hours > 0. {
                write!(buf, "{}h ", hours.trunc()).expect("battery hours left write error");
            }

            write!(buf, "{:02}m", minutes.trunc()).expect("battery time left write error");
        } else {
            write!(buf, "{}", EMPTY_OUTPUT).expect("battery time left write error");
        }
    }
}

fn battery() -> Battery {
    Manager::new().unwrap().batteries().unwrap().next().unwrap().unwrap()
}

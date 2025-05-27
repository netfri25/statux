use std::fmt::Write;

use anyhow::Context;
use battery::{Battery, Manager, State};

use super::{Component, EMPTY_OUTPUT};

pub struct BatteryLevel;

impl Component for BatteryLevel {
    async fn update(&mut self, buf: &mut String) -> anyhow::Result<()> {
        let battery = battery()?;
        let state = match battery.state() {
            State::Charging => '+',
            State::Discharging => '-',
            _ => 'o',
        };

        let charge = battery.state_of_charge().value * 100.;
        write!(buf, "{}{}", state, charge.trunc())?;
        Ok(())
    }
}

pub struct BatteryTimeLeft;

impl Component for BatteryTimeLeft {
    async fn update(&mut self, buf: &mut String) -> anyhow::Result<()> {
        let battery = battery()?;
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
                write!(buf, "{}h ", hours.trunc())?;
            }

            write!(buf, "{:02}m", minutes.trunc())?;
        } else {
            write!(buf, "{}", EMPTY_OUTPUT)?;
        }

        Ok(())
    }
}

fn battery() -> anyhow::Result<Battery> {
    let battery = Manager::new()?
        .batteries()?
        .next()
        .context("no battery")??;
    Ok(battery)
}

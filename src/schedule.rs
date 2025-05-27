use std::time::Duration;

use chrono::{DateTime, Datelike, TimeZone, Timelike};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Schedule(Duration);

#[allow(unused)]
impl Schedule {
    pub fn every() -> Self {
        Self::default()
    }

    pub fn milli(self) -> Self {
        self.millis(1)
    }

    pub fn millis(self, millis: u64) -> Self {
        Self(self.0 + Duration::from_millis(millis))
    }

    pub fn sec(self) -> Self {
        self.secs(1)
    }

    pub fn secs(self, secs: u64) -> Self {
        Self(self.0 + Duration::from_secs(secs))
    }

    pub fn min(self) -> Self {
        self.mins(1)
    }

    pub fn mins(self, mins: u64) -> Self {
        self.secs(mins * 60)
    }

    pub fn hour(self) -> Self {
        self.hours(1)
    }

    pub fn hours(self, hours: u64) -> Self {
        self.mins(60 * hours)
    }

    pub fn day(self) -> Self {
        self.days(1)
    }

    pub fn days(self, days: u64) -> Self {
        self.hours(24 * days)
    }

    pub fn after<Tz: TimeZone>(&self, time: DateTime<Tz>) -> DateTime<Tz> {
        let time_millis = {
            let mut total = time.timestamp_subsec_millis() as u128;
            total += time.second() as u128 * 1000;
            total += time.minute() as u128 * 1000 * 60;
            total += time.hour() as u128 * 1000 * 60 * 60;
            total += time.day0() as u128 * 1000 * 60 * 60 * 24;
            total
        };

        let interval_millis = self.0.as_millis();
        let next = time_millis.next_multiple_of(interval_millis);
        let delta = next - time_millis;

        time + chrono::Duration::milliseconds(delta.try_into().unwrap())
    }
}

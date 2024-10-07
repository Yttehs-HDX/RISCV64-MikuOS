use core::ops;
use crate::{config::CLOCK_FREQ, sbi};

const MILLIS_PER_SEC: usize = 1_000;
const MICRO_PER_SEC: usize = 1_000_000;
const TIGGER_TIME: usize = 100000;

pub fn get_current_time() -> TimeVal {
    let time = sbi::sbi_get_time();
    TimeVal::from_raw(time)
}

pub fn set_timer(timer: TimeVal) {
    sbi::sbi_set_timer(timer.as_sec(TimeType::Raw));
}

pub fn set_next_trigger() {
    let current_time = get_current_time();
    let next_time = TimeVal::new(0, TIGGER_TIME);
    set_timer(current_time + next_time);
}

// region TimeVal begin
pub struct TimeVal {
    time: usize,
}

impl ops::Add<TimeVal> for TimeVal {
    type Output = TimeVal;

    fn add(self, rhs: TimeVal) -> TimeVal {
        TimeVal {
            time: self.time + rhs.time,
        }
    }
}

impl ops::Sub<TimeVal> for TimeVal {
    type Output = TimeVal;

    fn sub(self, rhs: TimeVal) -> TimeVal {
        TimeVal {
            time: self.time - rhs.time,
        }
    }
}

impl ops::Mul<usize> for TimeVal {
    type Output = TimeVal;

    fn mul(self, rhs: usize) -> TimeVal {
        TimeVal {
            time: self.time * rhs,
        }
    }
}

impl ops::Div<usize> for TimeVal {
    type Output = TimeVal;

    fn div(self, rhs: usize) -> TimeVal {
        TimeVal {
            time: self.time / rhs,
        }
    }
}

impl TimeVal {
    pub fn new(sec: usize, usec: usize) -> Self {
        TimeVal {
            time: sec * CLOCK_FREQ + usec * CLOCK_FREQ / MICRO_PER_SEC,
        }
    }

    pub fn from_raw(time: usize) -> Self {
        TimeVal { time }
    }

    pub fn as_sec(&self, sec_type: TimeType) -> usize {
        match sec_type {
            TimeType::Raw => self.time,
            TimeType::Sec => self.time / CLOCK_FREQ,
            TimeType::MSec => self.time * MILLIS_PER_SEC / CLOCK_FREQ,
            TimeType::USec => self.time * MICRO_PER_SEC / CLOCK_FREQ,
        }
    }
}
// region TimeVal end

// region TimeType begin
#[allow(unused)]
pub enum TimeType {
    Raw,
    Sec,
    MSec,
    USec,
}
// region TimeType end
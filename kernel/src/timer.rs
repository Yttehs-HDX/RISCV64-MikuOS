use crate::{config::CLOCK_FREQ, sbi};
use core::fmt::Display;
use core::ops;

const MILLIS_PER_SEC: usize = 1_000;
const MICRO_PER_SEC: usize = 1_000_000;

const TIGGER_TIME: usize = 100_000; // 100 ms

pub fn get_current_time() -> TimeVal {
    let time = sbi::sbi_get_time();
    TimeVal::from_reg(time)
}

pub fn set_timer(timer: TimeVal) {
    let timer = timer.get_time(TimeUnit::Tick);
    sbi::sbi_set_timer(timer);
}

pub fn set_next_trigger() {
    let current_time = get_current_time();
    let next_time = TimeVal::new(0, TIGGER_TIME);
    set_timer(current_time + next_time);
}

// region TimeVal begin
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct TimeVal {
    sec: usize,
    usec: usize,
}

impl Display for TimeVal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:02}:{:02}.{:06}",
            self.sec / 60,
            self.sec % 60,
            self.usec
        )
    }
}

impl TimeVal {
    pub fn new(sec: usize, usec: usize) -> Self {
        TimeVal { sec, usec }
    }

    pub fn from_reg(time: usize) -> Self {
        let sec = time / CLOCK_FREQ;
        let usec = time % CLOCK_FREQ * MICRO_PER_SEC / CLOCK_FREQ;
        TimeVal { sec, usec }
    }

    pub fn get_time(&self, unit: TimeUnit) -> usize {
        match unit {
            TimeUnit::Hour => self.sec / 3600,
            TimeUnit::Min => self.sec / 60,
            TimeUnit::Sec => self.sec,
            TimeUnit::Msec => self.sec * MILLIS_PER_SEC + self.usec / MICRO_PER_SEC,
            TimeUnit::Usec => self.sec * MICRO_PER_SEC + self.usec,
            TimeUnit::Tick => self.sec * CLOCK_FREQ + self.usec * CLOCK_FREQ / MICRO_PER_SEC,
        }
    }
}

impl ops::Add<TimeVal> for TimeVal {
    type Output = TimeVal;

    fn add(self, rhs: TimeVal) -> TimeVal {
        let mut sec = self.sec + rhs.sec;
        let mut usec = self.usec + rhs.usec;
        if usec >= MICRO_PER_SEC {
            sec += 1;
            usec -= MICRO_PER_SEC;
        }
        TimeVal { sec, usec }
    }
}

impl ops::Sub<TimeVal> for TimeVal {
    type Output = TimeVal;

    fn sub(self, rhs: TimeVal) -> TimeVal {
        let mut sec = self.sec as isize - rhs.sec as isize;
        let mut usec = self.usec as isize - rhs.usec as isize;
        if usec < 0 {
            sec -= 1;
            usec += MICRO_PER_SEC as isize;
        }
        TimeVal {
            sec: sec as usize,
            usec: usec as usize,
        }
    }
}

impl ops::Mul<usize> for TimeVal {
    type Output = TimeVal;

    fn mul(self, rhs: usize) -> TimeVal {
        let mut sec = self.sec * rhs;
        let mut usec = self.usec * rhs;
        if usec >= MICRO_PER_SEC {
            sec += usec / MICRO_PER_SEC;
            usec %= MICRO_PER_SEC;
        }
        TimeVal { sec, usec }
    }
}

impl ops::Div<usize> for TimeVal {
    type Output = TimeVal;

    fn div(self, rhs: usize) -> TimeVal {
        let mut sec = self.sec / rhs;
        let mut usec = self.usec / rhs;
        if self.usec % rhs != 0 {
            let remainder = self.usec % rhs;
            sec += remainder * MICRO_PER_SEC / rhs;
            usec = (usec * MICRO_PER_SEC + remainder) / rhs;
        }
        TimeVal { sec, usec }
    }
}

impl PartialOrd for TimeVal {
    fn partial_cmp(&self, other: &TimeVal) -> Option<core::cmp::Ordering> {
        if self.sec < other.sec {
            Some(core::cmp::Ordering::Less)
        } else if self.sec > other.sec {
            Some(core::cmp::Ordering::Greater)
        } else if self.usec < other.usec {
            Some(core::cmp::Ordering::Less)
        } else if self.usec > other.usec {
            Some(core::cmp::Ordering::Greater)
        } else {
            Some(core::cmp::Ordering::Equal)
        }
    }
}
// region TimeVal end

// region TimeUnit begin
#[allow(unused)]
pub enum TimeUnit {
    Hour,
    Min,
    Sec,
    Msec,
    Usec,
    Tick,
}
// region TimeUnit end

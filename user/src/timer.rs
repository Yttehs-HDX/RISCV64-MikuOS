use core::ops;

const MILLIS_PER_SEC: usize = 1_000;
const MICRO_PER_SEC: usize = 1_000_000;

// region TimeVal begin
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct TimeVal {
    sec: usize,
    usec: usize,
}

impl TimeVal {
    pub fn empty() -> Self {
        TimeVal { sec: 0, usec: 0 }
    }

    pub fn new(sec: usize, usec: usize) -> Self {
        TimeVal { sec, usec }
    }

    pub fn get_time(&self, unit: TimeUnit) -> usize {
        match unit {
            TimeUnit::Hour => self.sec / 3600,
            TimeUnit::Min => self.sec / 60,
            TimeUnit::Sec => self.sec,
            TimeUnit::Msec => self.sec * MILLIS_PER_SEC + self.usec / MICRO_PER_SEC,
            TimeUnit::Usec => self.sec * MICRO_PER_SEC + self.usec,
        }
    }

    pub fn format(&self) -> &str {
        format_args!(
            "{:02}:{:02}:{:02}.{:06}",
            self.sec / 3600,
            self.sec % 3600 / 60,
            self.sec % 60,
            self.usec
        )
        .as_str()
        .unwrap()
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
}
// region TimeUnit end

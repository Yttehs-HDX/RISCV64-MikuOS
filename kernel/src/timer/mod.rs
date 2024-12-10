pub use time_val::*;

use crate::sbi;

mod time_val;

const TIGGER_TIME: usize = 100_000; // 100 ms

pub fn get_current_tick() -> usize {
    sbi::sbi_get_time()
}

pub fn get_current_time() -> TimeVal {
    let time = get_current_tick();
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

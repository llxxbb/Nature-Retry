use std::thread::sleep;
use std::time::Duration;

use cfg::*;

/// more recored less sleep
pub fn sleep_by_records(records: usize, last_sleep: u64) -> u64 {
    if records == 0 {
        let last_sleep = last_sleep << 1;
        if last_sleep >= MAX_SLEEP {
            let last_sleep = MAX_SLEEP;
        }
    } else {
        let last_sleep = last_sleep >> 1;
        if last_sleep <= MIN_SLEEP {
            let last_sleep = MIN_SLEEP;
        }
    }
    sleep(Duration::from_millis(last_sleep));
    return last_sleep;
}


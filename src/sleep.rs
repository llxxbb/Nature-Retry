use std::thread::sleep;
use std::time::Duration;

use cfg::*;

/// more recored less sleep
pub fn sleep_by_records(records: u32, last_sleep: u64) -> u64 {
    let sl = match records {
        0 => {
            let last_sleep = last_sleep << 1;
            if last_sleep >= *MAX_SLEEP {
                *MAX_SLEEP
            } else { last_sleep }
        }
        _ => {
            let last_sleep = last_sleep >> 1;
            if last_sleep <= *MIN_SLEEP {
                *MIN_SLEEP
            } else { last_sleep }
        }
    };
    sleep(Duration::from_millis(sl));
    return last_sleep;
}


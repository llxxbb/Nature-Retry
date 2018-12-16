#![feature(custom_attribute)]
#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]  // rocket

extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate nature_common;
extern crate nature_db;
#[cfg(test)] #[macro_use]extern crate rocket;
extern crate serde_json;
#[macro_use]
extern crate log;


use std::ops::Deref;

use cfg::*;
use delay::*;
use nature_common::*;
use nature_common::util::setup_logger;
use nature_db::*;
use sender::*;
use sleep::*;

lazy_static! {
    pub static ref TaskService: TaskDaoImpl = TaskDaoImpl{};
}

pub fn start() {
    dotenv::dotenv().ok();
    let _ = setup_logger();
    let mut last_delay: u64 = 0;
    loop {
        if let Ok(rs) = TaskService.get_overdue(&FIRST_RETRY_INTERVAL.to_string()) {
            let _ = rs.iter().map(|r| {
                let max_times = *MAX_RETRY_TIMES.deref();
                if (r.retried_times as usize) < max_times {
                    match send(r) {
                        Ok(_) => {
                            let delay = get_delay_by_times(r.retried_times);
                            let _ = TaskService.increase_times_and_delay(&r.task_id, delay);
                        }
                        Err(e) => {
                            let _ = TaskService.raw_to_error(&e, r);
                        }
                    }
                } else {
                    let _ = TaskService.raw_to_error(&NatureError::ConverterEnvironmentError(format!("rtried over max times : {}", max_times)), r);
                }
            });
            last_delay = sleep_by_records(rs.len() as u32, last_delay)
        } else {
            last_delay = sleep_by_records(0, last_delay)
        }
    }
}


pub mod cfg;
pub mod sleep;
pub mod sender;
mod delay;
#[cfg(test)]
pub mod web_mocker;

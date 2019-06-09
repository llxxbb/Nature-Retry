#![feature(custom_attribute)]
#![feature(plugin)]

extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate nature_common;
extern crate nature_db;
extern crate serde_json;


use std::ops::Deref;

use cfg::*;
use delay::*;
use nature_common::*;
use nature_common::util::setup_logger;
use nature_db::*;
use sender::*;
use sleep::*;

type TaskService = TaskDaoImpl;

pub fn start() {
    dotenv::dotenv().ok();
    let _ = setup_logger();
    let mut last_delay: u64 = 0;
    loop {
        debug!("start a new loop");
        if let Ok(rs) = TaskService::get_overdue(&FIRST_RETRY_INTERVAL.to_string()) {
            debug!("load tasks number: {}", rs.len());
            let _ = rs.iter().for_each(|r| {
                debug!("process task: {:?}", r);
                let max_times = *MAX_RETRY_TIMES.deref();
                if (r.retried_times as usize) < max_times {
                    match send(r) {
                        Ok(_) => {
                            debug!("send task succeed!");
                            let delay = get_delay_by_times(r.retried_times);
                            let _ = TaskService::increase_times_and_delay(&r.task_id, delay);
                        }
                        Err(e) => {
                            debug!("send task failed!");
                            let _ = TaskService::raw_to_error(&e, r);
                        }
                    }
                } else {
                    debug!("tried too many times!");
                    let _ = TaskService::raw_to_error(&NatureError::ConverterEnvironmentError(format!("rtried over max times : {}", max_times)), r);
                }
            });
            last_delay = sleep_by_records(rs.len() as u32, last_delay)
        } else {
            debug!("no task found, sleep!");
            last_delay = sleep_by_records(0, last_delay)
        }
    }
}


pub mod cfg;
pub mod sleep;
pub mod sender;
mod delay;

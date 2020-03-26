#![feature(plugin)]

extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate nature_common;
extern crate nature_db;
extern crate serde_json;


use std::env;
use std::ops::Deref;

use reqwest::blocking::Client;

use cfg::*;
use delay::*;
use nature_common::*;
use nature_db::*;
use sleep::*;

type TaskService = TaskDaoImpl;
lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub fn start() {
    dotenv::dotenv().ok();
    let _ = setup_logger();
    let mut last_delay: u64 = 0;
    let base_delay = env::var("ASE_DELAY").unwrap_or_else(|_| "2".to_string()).parse::<i64>().unwrap();
    let load_size = env::var("LOAD_SIZE").unwrap_or_else(|_| "100".to_string()).parse::<i64>().unwrap();
    let clean_delay = env::var("CLEAN_DELAY").unwrap_or_else(|_| "2".to_string()).parse::<i64>().unwrap();
    loop {
        last_delay = once(last_delay, base_delay, load_size, clean_delay)
    }
}

fn once(last_delay: u64, base_delay: i64, limit: i64, finish_delay: i64) -> u64 {
    debug!("start a new loop");
    let mut len = 0;
    let rs = TaskService::get_overdue(base_delay, limit);
    match rs {
        Ok(rs) => {
            len = rs.len();
            debug!("load tasks number: {}", rs.len());
            let _ = rs.iter().for_each(|r| process_delayed(&r));
        }
        Err(e) => {
            warn!("found error: {}", e)
        }
    }
    match TaskService::delete_finished(finish_delay) {
        Ok(num) => info!("cleaned tasks : {}", num),
        Err(e) => warn!("clean task failed: {}", e)
    }
    sleep_by_records(len as u32, last_delay)
}


fn process_delayed(r: &&RawTask) -> () {
    debug!("process task: {:?}", r);
    let max_times = *MAX_RETRY_TIMES.deref();
    if (r.retried_times as usize) < max_times {
        let req = CLIENT.post(&*NATURE_SERVER_ADDRESS).json(r).send();
        match req {
            Ok(_) => {
                debug!("send task succeed!");
                let delay = get_delay_by_times(r.retried_times);
                // 注释掉下一行可用于并发测试
                if let Err(e) = TaskService::increase_times_and_delay(&r.task_id, delay) {
                    warn!("task update failed: {}", e);
                }
            }
            Err(_) => {
                warn!("send task failed!");
            }
        }
    } else {
        debug!("tried too many times!");
        let _ = TaskService::raw_to_error(&NatureError::EnvironmentError(format!("rtried over max times : {}", max_times)), r);
    }
}


pub mod cfg;
pub mod sleep;
mod delay;


// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn once_test() {
//         dotenv::dotenv().ok();
//         let _ = setup_logger();
//         once(0, 0, 1, 0);
//     }
// }
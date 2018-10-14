extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate nature_common;
extern crate nature_db;

use cfg::*;
use nature_common::*;
use nature_db::*;
use sender::*;
use sleep::*;
use std::env;
use std::ops::Deref;

lazy_static! {
    pub static ref DeliveryService: DeliveryDaoImpl = DeliveryDaoImpl{};
}

pub fn start() {
    dotenv::dotenv().ok();
    let overtime = env::var("OVERTIME").unwrap_or_else(|_| "5".to_string());
    loop {
        if let Ok(rs) = DeliveryService.get_overdue(&overtime) {
            let _ = rs.iter().map(|r| {
                let max_times = *MAX_RETRY_TIMES.deref();
                if (r.retried_times as usize) < max_times {
                    send(r);
                    let _ = DeliveryService.increase_times_and_delay(r.id.clone(), delay);
                } else {
                    let _ = DeliveryService.raw_to_error(&NatureError::ConverterEnvironmentError(format!("rtried over max times : {}", max_times)), r);
                }
            });
            sleep_by_records(rs.len())
        }
        else {
            sleep_by_records(0)
        }
    }
}

pub mod cfg;
pub mod sleep;
pub mod sender;


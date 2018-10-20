extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate nature_common;
extern crate nature_db;
extern crate serde_json;

use cfg::*;
use delay::*;
use nature_common::*;
use nature_db::*;
use sender::*;
use sleep::*;
use std::ops::Deref;
use nature_common::util::setup_logger;


lazy_static! {
    pub static ref DeliveryService: DeliveryDaoImpl = DeliveryDaoImpl{};
}

pub fn start() {
    dotenv::dotenv().ok();
    let _ = setup_logger();
    loop {
        if let Ok(rs) = DeliveryService.get_overdue(&FIRST_RETRY_INTERVAL.to_string()) {
            let _ = rs.iter().map(|r| {
                let max_times = *MAX_RETRY_TIMES.deref();
                if (r.retried_times as usize) < max_times {
                    match serde_json::to_string(r) {
                        Ok(json) => {
                            send(json);
                            let delay = get_delay_by_times(r.retried_times);
                            let _ = DeliveryService.increase_times_and_delay(&r.id, delay);
                        }
                        Err(e) => {
                            let _ = DeliveryService.raw_to_error(&NatureError::from(e), r);
                        }
                    }
                } else {
                    let _ = DeliveryService.raw_to_error(&NatureError::ConverterEnvironmentError(format!("rtried over max times : {}", max_times)), r);
                }
            });
            sleep_by_records(rs.len())
        } else {
            sleep_by_records(0)
        }
    }
}


pub mod cfg;
pub mod sleep;
pub mod sender;
mod delay;


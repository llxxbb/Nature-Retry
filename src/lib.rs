extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate nature_common;
extern crate nature_db;

use cfg::*;
use nature_common::*;
use nature_db::*;
use overdue::*;
use sender::*;
use sleep::*;
use std::ops::Deref;


type DeliveryService = DeliveryDaoImpl;

pub fn start() {
    dotenv::dotenv().ok();

    loop {
        match get_overdue() {
            None => sleep_by_records(0),
            Some(rs) => {
                let _ = rs.iter().map(|r| {
                    let max_times = *MAX_RETRY_TIMES.deref();
                    if (r.retried_times as usize) < max_times {
                        send(r);
                        let _ = DeliveryService::increase_times(r.id.clone());
                    } else {
                        let _ = DeliveryService::raw_to_error(&NatureError::ConverterEnvironmentError(format!("rtried over max times : {}", max_times)), r);
                    }
                });
                sleep_by_records(rs.len())
            }
        }
    }
}

pub mod cfg;
pub mod sleep;
pub mod overdue;
pub mod sender;


extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate nature_common;
extern crate nature_db;

use sleep::*;
use overdue::*;
use sender::*;
use nature_db::*;


type DeliveryService = DeliveryDaoImpl;

pub fn start() {
    dotenv::dotenv().ok();

    loop {
        // read overdue
        match get_overdue() {
            None => sleep_by_records(0),
            Some(rs) => {
                let _ = rs.iter().map(|r|{
                    send(r);
                    DeliveryService::increase_times(r.id.clone())

                    // sleep a while
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


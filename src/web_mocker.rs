extern crate rocket_contrib;

use nature_common::*;
use nature_common::util::setup_logger;
use nature_db::RawTask;
use rocket::{ignite, Rocket};
use self::rocket_contrib::json::Json;
use std::thread;

pub fn rocket_server() -> Rocket {
    ignite()
        .mount("/", routes![redo_task_ok])
        .mount("/", routes![redo_task_false])
}

#[post("/redo_task_ok", format = "application/json", data = "<_task>")]
fn redo_task_ok(_task: Json<RawTask>) -> Json<Result<()>> {
    Json(Ok(()))
}

#[post("/redo_task_false", format = "application/json", data = "<_task>")]
fn redo_task_false(_task: Json<RawTask>) -> Json<Result<()>> {
    Json(Err(NatureError::SystemError("too busy".to_string())))
}


pub fn test_env_init() {
    let _ = setup_logger();
    thread::spawn(|| {
        let _ = rocket_server().launch();
    });
}
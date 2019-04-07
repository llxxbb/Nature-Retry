extern crate reqwest;

use std::result::Result as RT;
use std::thread::sleep;
use std::time::Duration;

use crate::cfg::*;
use nature_common::*;
use nature_db::RawTask;

use self::reqwest::{Client, Error};

lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub fn send(task: &RawTask) -> Result<()> {
    loop {
        let req = CLIENT.post(&*NATURE_SERVER_ADDRESS).json(task);
        let req = req.build();
        match req {
            Err(e) => return Err(NatureError::VerifyError(e.to_string())),
            Ok(req) => match CLIENT.execute(req) {
                Ok(mut res) => {
                    let rtn: RT<Result<()>, Error> = res.json();
                    match rtn {
                        Ok(_) => break,
                        Err(_) => sleep(Duration::from_secs(*BUSY_SLEEP))
                    }
                }
                Err(_err) => sleep(Duration::from_secs(*BUSY_SLEEP))
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    extern crate serde_json;

    use std::env;
    use std::thread;
    use std::time::Duration;

    use nature_db::*;
    use crate::web_mocker::test_env_init;

    use super::*;

    #[test]
    #[allow(dead_code)]
    fn test_send_ok() {
        test_env_init();
        thread::sleep(Duration::from_millis(100));
        env::set_var("NATURE_SERVER_ADDRESS", "http://localhost:8080/redo_task_ok");
        let task = RawTask::new(&"hello".to_string(), "/hello", 2).unwrap();
        let _ = send(&task);
    }

    #[test]
    #[allow(dead_code)]
    fn test_send_false() {
        test_env_init();
        thread::sleep(Duration::from_millis(100));
        env::set_var("NATURE_SERVER_ADDRESS", "http://localhost:8080/redo_task_false");
        let task = RawTask::new(&"hello".to_string(), "/hello", 2).unwrap();
        let _ = send(&task);
    }
}
use std::env;

lazy_static! {
    pub static ref MAX_RETRY_TIMES : usize = {
        env::var("MAX_RETRY_TIMES").unwrap_or("6".to_string()).parse::<usize>().unwrap()
    };
    pub static ref FIRST_RETRY_INTERVAL : usize = {
        env::var("FIRST_RETRY_INTERVAL").unwrap_or("5".to_string()).parse::<usize>().unwrap()
    };
    pub static ref MAX_SLEEP : usize = {
        env::var("MAX_SLEEP").unwrap_or("60000".to_string()).parse::<usize>().unwrap()
    };
    pub static ref NATURE_SERVER_ADDRESS : String = {
        env::var("NATURE_SERVER_ADDRESS").unwrap_or("http://localhost:8080/redo_task".to_string())
    };
}

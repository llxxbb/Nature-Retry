extern crate nature_retry;

use nature_retry::*;

#[tokio::main]
pub async fn main() {
    start().await
}
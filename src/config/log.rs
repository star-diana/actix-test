extern crate env_logger;

use std::env;

use dotenv;

pub fn init_logger() {
    env::set_var("RUST_LOG", dotenv::var("RUST_LOG").unwrap());
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

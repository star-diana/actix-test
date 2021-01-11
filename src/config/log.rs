extern crate log4rs;

use std::env;

use dotenv;

pub fn init_logger() {
    env::set_var("RUST_LOG", dotenv::var("RUST_LOG").unwrap());
    env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log.yaml",Default::default()).unwrap();
}

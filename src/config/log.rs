extern crate log4rs;

use std::env;

pub fn init_logger() {
    env::set_var("RUST_BACKTRACE", "full");
    log4rs::init_file("log.yaml",Default::default()).unwrap();
}

extern crate env_logger;

use dotenv;

pub fn init_logger() {
    std::env::set_var("RUST_LOG", dotenv::var("RUST_LOG").unwrap());
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

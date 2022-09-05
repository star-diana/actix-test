use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use crate::config::CONFIG;

pub fn init_pool() -> Rbatis {
    println!("rbatis pool init ({})...", CONFIG.DB_URL);
    let rbatis = Rbatis::new();
    rbatis
        .init(MysqlDriver {}, CONFIG.DB_URL.as_str())
        .expect("rbatis pool init fail!");

    rbatis
}

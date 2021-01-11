use app_config::ApplicationConfig;

pub mod router;
pub mod log;
pub mod db;
pub mod app_config;

pub use router::router;


//当前服务配置
lazy_static! {
      pub static ref CONFIG: ApplicationConfig = ApplicationConfig::default();
}

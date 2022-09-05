use std::io;
use actix_cors::Cors;
use actix_web::{
    web, App, HttpServer,
    middleware::{DefaultHeaders},
};
// use actix_web::error::{InternalError};
use log::{info};
use actix_test::config::{log as Log, router, database, CONFIG, application};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 日志初始化
    // Log::init_logger();
    // 数据库连接初始化
    // init_db_link().await;

    let local_ip = local_ipaddress::get().unwrap();

    info!("Actix-web App Running :");
    info!(" - Local:\thttp://localhost:{}", &CONFIG.PORT);
    info!(" - Network:\thttp://{}:{}", local_ip, &CONFIG.PORT);

    let data = web::Data::new(application::ApplicationState {
        rbatis: database::init_pool(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            // 小型中间件写法，正式环境不建议使用
            // .wrap_fn(|req, srv| {
            //     println!("Hi from start. You requested: {}", req.path());
            //     srv.call(req).map(|res| {
            //         println!("Hi from response");
            //         res
            //     })
            // })
            // 日志中间件
            // .wrap(middleware::Logger::default())
            // CORS 中间件
            .wrap(Cors::permissive())
            // 默认响应的头部的中间件
            .wrap(DefaultHeaders::new().add(("X-Server-Version", "0.1")))
            // 配置路由
            .configure(router::router)
    })
        .bind(format!("{}:{}", CONFIG.BIND_HOST, CONFIG.PORT))?
        .run()
        .await
}

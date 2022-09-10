use actix_cors::Cors;
use actix_web::{
    middleware::{DefaultHeaders, Logger},
    web, App, HttpServer,
};
use std::io;
use log::info;
use actix_test::{database, app_log, router, ApplicationState, CONFIG};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 日志初始化
    app_log::init_logger();
    // app 状态初始化
    let data = web::Data::new(ApplicationState {
        rbatis: database::init_pool(),
    });

    let local_ip = local_ipaddress::get().unwrap();

    info!("Actix-web App Running :");
    info!(" - Local:    http://localhost:{}", &CONFIG.PORT);
    info!(" - Network:  http://{}:{}", local_ip, &CONFIG.PORT);

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
            .wrap(Logger::default())
            // 默认响应的头部的中间件
            .wrap(DefaultHeaders::new().add(("X-Server-Version", "0.1")))
            // CORS 中间件
            .wrap(Cors::permissive())
            // 配置路由
            .configure(router::router)
    })
        .bind(format!("{}:{}", CONFIG.BIND_HOST, CONFIG.PORT))?
        .run()
        .await
}

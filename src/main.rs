use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, middleware};
// use actix_web::error::{InternalError};

use local_ipaddress;
use log::{info};

use actix_test::config::{log as Log, router, db, CONFIG};

async fn init_db_link() {
    let mysql_url = format!("mysql://{}:{}@{}:{}/{}?{}", CONFIG.db_username, CONFIG.db_password, CONFIG.db_host, CONFIG.db_port, CONFIG.db_name, CONFIG.db_query_str);
    let pgsql_url = format!("postgresql://{}:{}@{}:{}/{}?{}", CONFIG.db_username, CONFIG.db_password, CONFIG.db_host, CONFIG.db_port, CONFIG.db_name, CONFIG.db_query_str);
    let url = match CONFIG.db_type.as_str() {
        "mysql" => mysql_url,
        "postgresql" => pgsql_url,
        _ => mysql_url,
    };
    db::RB.link(&url).await.unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 日志初始化
    Log::init_logger();
    // 数据库连接初始化
    init_db_link().await;

    let local_ip = local_ipaddress::get().unwrap();

    info!("actix-web app run at:");
    info!("Local:\thttp://127.0.0.1:{}", CONFIG.app_port);
    info!("Network:\thttp://{}:{}", local_ip, CONFIG.app_port);

    HttpServer::new(|| {
        // 配置json提取器
        // let json_config = web::JsonConfig::default()
        //     .limit(4096)
        //     .error_handler(|err, _req| {
        //         // 创建一个自定义的错误类型
        //         InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
        //     });

        App::new()
            // 日志中间件
            .wrap(middleware::Logger::default())
            // 压缩中间件，默认情况下 ContentEncoding::Auto 被使用
            .wrap(middleware::Compress::default())
            // 小型中间件写法，正式环境不建议使用
            // .wrap_fn(|req, srv| {
            //     println!("Hi from start. You requested: {}", req.path());
            //     srv.call(req).map(|res| {
            //         println!("Hi from response");
            //         res
            //     })
            // })
            // CORS 中间件
            .wrap(Cors::permissive())
            // 默认响应的头部的中间件
            .wrap(middleware::DefaultHeaders::new().header("X-App-Version", "0.1"))
            // 配置路由
            .configure(router)
    })
        .bind(format!("{}:{}", CONFIG.app_bind_host, CONFIG.app_port))?
        .run()
        .await
}

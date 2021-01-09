use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, middleware, web};
use actix_web::error::{InternalError};

use dotenv;
use local_ipaddress;
use log::{debug, error, info};

use actix_web_test::config::{log as Log, router};
use actix_web_test::util::error::CustomError;

#[get("/ee")]
async fn ee() -> Result<String, CustomError> {
    let error = CustomError::ValidationError { message: String::from("啦啦啦啦") };
    let result = Err(error);

    Ok(result.map_err(|e| e)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Log::init_logger();

    let local_ip = local_ipaddress::get().unwrap();
    let host = dotenv::var("APP_BASIC_HOST").unwrap();
    let port = dotenv::var("PORT").unwrap();

    info!("actix-web app run at:");
    info!("- Local:\thttp://127.0.0.1:{}", port);
    info!("- Network:\thttp://{}:{}", local_ip, port);

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
            .service(ee)
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}

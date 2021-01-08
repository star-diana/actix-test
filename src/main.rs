extern crate derive_more;

use actix_web::{App, dev::HttpResponseBuilder, Error, get, HttpRequest, HttpResponse, HttpServer, middleware, web};
use actix_web::error::{InternalError, ResponseError};
use actix_web::http::{header, StatusCode};
use derive_more::{Display as MoreDisplay, Error as MoreError};
use dotenv;
use local_ipaddress;
use log::{debug, error, info};

use actix_web_test::config::{log as Log, router};

#[get("/ee")]
async fn ee() -> Result<String, UserError> {
    let error = UserError::ValidationError { field: String::from("啦啦啦啦") };
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
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // 创建一个自定义的错误类型
                InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new()
            // 使用日志中间件
            .wrap(middleware::Logger::default())
            // 压缩中间件
            // 默认情况下 ContentEncoding::Auto 被使用
            .wrap(middleware::Compress::default())
            // 设置默认响应的头部的中间件
            .wrap(middleware::DefaultHeaders::new().header("X-App-Version", "0.1"))
            // 配置路由
            .configure(router)
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}

#[derive(Debug, MoreDisplay, MoreError)]
enum UserError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

// 为自定义错误实现 ResponseError 以可返回 HTTP 错误
impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
}

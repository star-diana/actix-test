use crate::service::user::{get_info, hello, payload, get_all_users, get_user, add_new_user, update_user, del_user};

use actix_web::{Error, web, HttpResponse};
use actix_web::web::{scope, ServiceConfig, resource};
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use log::{debug};


pub fn router(config: &mut ServiceConfig) {
    config
        .service(hello)
        .service(
            scope("/api/v1/order")
                .wrap(HttpAuthentication::bearer(validator))
                .service(payload)
        )
        .service(
            scope("/api/v1/user")
                // 身份验证中间件
                // 不能写在 main 那里，那里会拦截全部请求
                // 这里对此 scope 下的所有路由起作用
                // .wrap(HttpAuthentication::bearer(validator))
                // .service(get_user)
                .service(get_all_users)
                .service(get_info)
                .service(get_user)
                .service(add_new_user)
                .service(update_user)
                .service(del_user)
        )
        .service(
            scope("/api/v1")
                .service(resource("/login").route(web::get().to(|| HttpResponse::Ok().body("这是登录接口"))))
        );
}

// 身份验证具体处理方法
async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    debug!("{}", credentials.token().to_string());
    Ok(req)
}

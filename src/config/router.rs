use actix_web::web::{scope, ServiceConfig, resource};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::service::user::{get_info, hello, payload, get_all_users, get_user, add_new_user, update_user, del_user};
use crate::service::auth::login;
use crate::util::utils::validator;

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
                .wrap(HttpAuthentication::bearer(validator))
                .service(get_all_users)
                .service(get_user)
                .service(add_new_user)
                .service(update_user)
                .service(del_user)
        )
        .service(
            scope("/api/v1")
                .service(login)
        );
}

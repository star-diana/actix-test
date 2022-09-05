// use actix_web::web;
use actix_web::web::{ServiceConfig};
// use actix_web_httpauth::middleware::HttpAuthentication;

use crate::service::user;
// use crate::util::utils::validator;

pub fn router(config: &mut ServiceConfig) {
    config
        .service(user::hello)
        .service(user::get_info)
        .service(user::form)
        .service(user::get_api_info)
        .service(user::state);
    // .service(
    //     scope("/api/v1/order")
    //         .wrap(HttpAuthentication::bearer(validator))
    //         .service(payload)
    // )
    // .service(
    //     scope("/api/v1/user")
    //         // 身份验证中间件
    //         // 不能写在 main 那里，那里会拦截全部请求
    //         // 这里对此 scope 下的所有路由起作用
    //         .wrap(HttpAuthentication::bearer(validator))
    //         .service(get_all_users)
    //         .service(get_user)
    //         .service(add_new_user)
    //         .service(update_user)
    //         .service(del_user)
    // );
    // .service(
    //     scope("/api/v1")
    //         .service(login)
    // );
}

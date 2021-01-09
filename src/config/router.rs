use std::collections::HashMap;

use actix_web::web::{scope, ServiceConfig};

use crate::service::user::{echo, hello};

pub fn router(config: &mut ServiceConfig) {
    config
        .service(hello)
        .service(
            scope("/api/v1/user")
                .service(echo)
            // TODO
        );
    // .service(scope("/api/v2")
    //     .service(r("/user/{user_id}").route(get().to(get_user)))
    // );
}

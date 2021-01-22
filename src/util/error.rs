extern crate derive_more;

use actix_web::{HttpResponse, ResponseError};
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{StatusCode};
use derive_more::{Display, Error};
use serde::{Serialize};


#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "校验错误: {}", message)]
    ValidationError { message: String },
    #[display(fmt = "{}: {}", error, message)]
    UnauthorizedError {
        realm: String,
        error: String,
        message: String,
    },
    #[display(fmt = "登录失败: {}", message)]
    LoginError { message: String },
    #[display(fmt = "服务器异常: {}", message)]
    InternalError { message: String },
}

#[derive(Serialize)]
struct ResponseMessage {
    code: u32,
    message: String,
}

// 为自定义错误实现 ResponseError 以可返回 HTTP 错误
impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            CustomError::UnauthorizedError { .. } => StatusCode::UNAUTHORIZED,
            CustomError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::LoginError { .. } => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let mut builder = HttpResponseBuilder::new(status_code.clone());

        match self {
            CustomError::UnauthorizedError { realm, error, message } => {
                builder.header("WWW-Authenticate", format!("Bearer realm=\"{}\", error=\"{}\", error_description=\"{}\"", realm, error, message));
            }
            _ => {}
        }

        // let error_message = json!({"code":status_code.as_u16(),"message":self.to_string()});

        builder
            // .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            // .body(format!("{}", error_message))
            .json(ResponseMessage { code: status_code.as_u16() as u32, message: self.to_string() })
    }
}

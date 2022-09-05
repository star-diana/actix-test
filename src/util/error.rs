use actix_web::{
    error, get,
    http::{ header::ContentType, StatusCode },
    App, HttpResponse, HttpServer,
};
use derive_more::{ Display, Error };
use serde::{ Serialize };

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "校验错误: {}", message)]
    ValidationError { message: &'static str },

    #[display(fmt = "{}: {}", error, message)]
    UnauthorizedError {
        realm: &'static str,
        error: &'static str,
        message: &'static str,
    },

    #[display(fmt = "登录失败: {}", message)]
    LoginError { message: &'static str },

    #[display(fmt = "服务器异常: {}", message)]
    InternalError { message: &'static str },
}

#[derive(Serialize)]
struct ResponseMessage {
    code: u16,
    message: String,
}

// 为自定义错误实现 ResponseError 以可返回 HTTP 错误
impl error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            CustomError::UnauthorizedError { .. } => StatusCode::UNAUTHORIZED,
            CustomError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::LoginError { .. } => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut builder = HttpResponse::build(self.status_code());

        match self {
            CustomError::UnauthorizedError { realm, error, message } => {
                builder.insert_header((
                    "WWW-Authenticate",
                    format!("Bearer realm=\"{}\", error=\"{}\", error_description=\"{}\"", realm, error, message)
                ));
            }
            _ => {}
        }

        // let error_message = json!({"code":status_code.as_u16(),"message":self.to_string()});

        builder.insert_header(ContentType::json())
            // .body(format!("{}", error_message))
            .json(ResponseMessage {
                code: self.status_code().as_u16(),
                message: self.to_string(),
            })
    }
}

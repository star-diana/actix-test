use actix_web::{
    http::{ header::ContentType, StatusCode },
    error, HttpResponse,
};
use serde::{ Serialize };

#[derive(Debug, derive_more::Display, derive_more::Error)]
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

        if let CustomError::UnauthorizedError { realm, error, message } = self {
            builder.insert_header((
                "WWW-Authenticate",
                format!("Bearer realm=\"{}\", error=\"{}\", error_description=\"{}\"", realm, error, message)
            ));
        }

        builder
            .insert_header(ContentType::json())
            .json(ResponseMessage {
                code: self.status_code().as_u16(),
                message: self.to_string(),
            })
    }
}

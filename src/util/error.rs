extern crate derive_more;

use actix_web::{HttpResponse, ResponseError};
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum CustomError {
    #[display(fmt = "Validation error on field: {}", message)]
    ValidationError { message: String },
    #[display(fmt = "Bearer realm=\"{}\", error=\"{}\", error_description=\"{}\"", realm, error, message)]
    UnauthorizedError {
        realm: String,
        error: String,
        message: String,
    },
}

// 为自定义错误实现 ResponseError 以可返回 HTTP 错误
impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            CustomError::UnauthorizedError { .. } => StatusCode::UNAUTHORIZED
        }
    }
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let mut builder = HttpResponseBuilder::new(status_code);

        match self {
            CustomError::UnauthorizedError { realm, error, message } => {
                builder.header("WWW-Authenticate", format!("Bearer realm=\"{}\", error=\"{}\", error_description=\"{}\"", realm, error, message));
            }
            _ => {}
        }

        builder
            .set_header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(self.to_string())
    }
}

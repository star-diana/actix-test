use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::{Error, error};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::HeaderValue;
use futures::future::{ok, Ready};
use futures::Future;

use crate::util::error::CustomError;

/// 在中间件处理过程器有两步.
/// 1. 中间件初始化, 下一个服务链中作为一个参数中间件工厂被调用.
/// 2. 中间件的调用方法被正常的请求调用.
pub struct Auth;

///中间件工厂是来自 actix_service 包下的一个 `Transform` trait.
/// `S` - 下一个服务类型
/// `B` - 响应body类型
impl<S, B> Transform<S> for Auth
    where S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
          S::Future: 'static,
          B: 'static {
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(RefCell::new(service))
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for AuthMiddleware<S>
    where S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
          S::Future: 'static,
          B: 'static {
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // println!("Hi from start. You requested: {}", req.path());
        let mut s = self.service.clone();
        // let future = self.service.call(req);
        let default = HeaderValue::from_str("").unwrap();
        let token = req.headers().get("Authorization").unwrap_or(&default);

        Box::pin(async move {
            // println!("Hi from response");

            // 这里是在路由匹配之前
            // 这里要配置需要拦截的路由，不然所有的请求都会被拦截
            if token.is_empty() && !req.path().eq("/login") {
                Err(
                    CustomError::UnauthorizedError {
                        realm: "aili.moe".to_string(),
                        error: "expired".to_string(),
                        message: "token expired".to_string(),
                    }.into()
                )
            } else {
                Ok(s.call(req).await?)
            }
        })
    }
}

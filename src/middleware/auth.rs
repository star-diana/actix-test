use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::Error;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::{ok, Ready};
use futures::Future;

/// 在中间件处理过程器有两步.
/// 1. 中间件初始化, 下一个服务链中作为一个参数中间件工厂被调用.
/// 2. 中间件的调用方法被正常的请求调用.
pub struct Auth;

///中间件工厂是来自 actix_service 包下的一个 `Transform` trait.
/// `S` - 下一个服务类型
/// `B` - 响应body类型
impl<S, B> Transform<S> for Auth
    where S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
          S::Future: 'static,
          B: 'static {
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
    where S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
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
        println!("Hi from start. You requested: {}", req.path());

        let future = self.service.call(req);

        Box::pin(async move {
            let result = future.await?;

            println!("Hi from response");
            Ok(result)
        })
    }
}

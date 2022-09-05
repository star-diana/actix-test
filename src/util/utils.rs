use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web::http::header;
use actix_web_httpauth::extractors::bearer::BearerAuth;

use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm, Validation, decode, DecodingKey};

use crate::config::CONFIG;
use crate::model::Claims;
// use crate::model::user::User;
use crate::util::error::CustomError;


// // 身份验证具体处理方法
// pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
//     let token = credentials.token();
//     let mut validation = Validation::new(Algorithm::HS384);
//     validation.set_issuer(&[&CONFIG.token_issuer]);
//     let result = decode::<Claims>(token, &DecodingKey::from_secret(CONFIG.token_secret.as_ref()), &validation)
//         .map_err(|e| {
//             let host = &*req.headers().get(header::HOST).unwrap().to_str().unwrap();
//             CustomError::UnauthorizedError {
//                 realm: host,
//                 error: "Unauthorized",
//                 message: &*e.to_string(),
//             }
//         })?;
//     debug!("{:?}", result.claims);
//
//     Ok(req)
// }

// 生成 token
// pub fn sign_token(user: User) -> Result<String, Error> {
//     let next_week = Utc::now() + Duration::days(7);
//
//     debug!("过期时间 ==> {:?}", next_week);
//     let claims = Claims {
//         exp: next_week.timestamp() as usize,
//         iss: &*CONFIG.token_issuer,
//         id: user.uid as usize,
//         uname: user.uname,
//     };
//
//     let token = encode(&Header::new(Algorithm::HS384), &claims, &EncodingKey::from_secret(CONFIG.token_secret.as_ref()))
//         .map_err(|e| CustomError::InternalError { message: e.to_string().as_str() })?;
//
//     Ok(token)
// }

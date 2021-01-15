use serde::{Deserialize, Serialize};
use jsonwebtoken::Algorithm;
use std::collections::HashSet;

pub mod user;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    // 必要，过期时间，UTC 时间戳
    pub exp: usize,
    // 可选，签发人
    pub iss: String,
    pub id: usize,
    pub uname: String,
}

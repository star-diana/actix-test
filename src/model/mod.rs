use serde::{Deserialize, Serialize};

pub mod user;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    // 必要，过期时间，UTC 时间戳
    pub exp: usize,
    // 可选，签发人
    pub iss: &'static str,
    pub id: usize,
    pub uname: &'static str,
}

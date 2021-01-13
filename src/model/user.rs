use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[crud_enable(id_name: "uid" | id_type: "u32")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub uid: u32,
    pub uname: String,
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub created_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
}

#[crud_enable(id_name: "uid" | id_type: "u32" | table_name: "user")]
#[derive(Clone, Debug)]
pub struct NewUser {
    pub uname: String,
    pub nickname: String,
    pub password: String,
    pub email: String,
}

#[crud_enable(id_name: "uid" | id_type: "u32" | table_name: "user")]
#[derive(Clone, Debug)]
pub struct UpdateUser {
    pub uid: u32,
    pub uname: String,
    pub nickname: String,
    pub password: String,
    pub email: String,
}

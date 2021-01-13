use actix_web::{Either, error, Error, get, post, put, HttpRequest, HttpResponse, Responder, web};
use actix_web::http::Version;
use actix_web::web::Path;

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use rbatis::crud::CRUD;
use rbatis::plugin::page::{PageRequest, Page};
use serde_urlencoded::from_str;
use std::collections::HashMap;

use crate::config::db::RB;
use crate::model::user::{User, NewUser, UpdateUser};
use crate::util::error::CustomError;
use actix_web::dev::Service;

#[derive(Deserialize, Serialize)]
pub struct Info {
    name: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Actix-Web!")
}

#[derive(Deserialize, Serialize)]
pub struct Pagination {
    page: u64,
}

#[get("/all")]
pub async fn get_all_users(req: HttpRequest) -> Result<HttpResponse, Error> {
    debug!("query string: {}", req.query_string());
    let query_map = from_str::<HashMap<String, String>>(req.query_string()).unwrap();
    let page = query_map
        .get("page")
        .unwrap_or(&String::from("1"))
        .parse::<u64>()
        .map_err(|_e| CustomError::ValidationError { message: "\"page\"参数必须是数字".to_string() })?;
    let page_size = query_map
        .get("page_size")
        .map(|s| { if s.is_empty() { "20" } else { s } })
        .unwrap_or(&String::from("20"))
        .parse::<u64>()
        .map_err(|_e| CustomError::ValidationError { message: "\"page_size\"参数必须是数字".to_string() })?;

    let request = PageRequest::new(page, page_size);
    let wrapper = RB.new_wrapper().check().unwrap();
    let result = RB.fetch_page_by_wrapper::<User>("", &wrapper, &request)
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?;

    // 不分页
    // let result = RB.list::<User>("").await.map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/{id}")]
pub async fn get_user(Path(id): Path<u32>) -> Result<HttpResponse, Error> {
    //  wrapper 写法
    // let wrapper = RB.new_wrapper().eq("uid", id).check().unwrap();
    // let user = RB.fetch_by_wrapper::<Option<User>>("", &wrapper)
    //     .await
    //     .map_err(|e| error::ErrorInternalServerError(e))?;

    let user = RB.fetch_by_id::<Option<User>>("", &id)
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?;

    debug!("{:?}", user);
    Ok(HttpResponse::Ok().json(user))
}

#[post("")]
pub async fn add_new_user(user: web::Json<NewUser>) -> Result<HttpResponse, Error> {
    debug!("{:?}", user.0);
    let result = RB.save("", &user.0)
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(result.last_insert_id.unwrap().to_string()))
}


#[put("/{id}")]
pub async fn update_user(Path(id): Path<u32>, user: web::Json<UpdateUser>) -> Result<HttpResponse, Error> {
    if !id.eq(&user.0.uid) {
        return Err(CustomError::ValidationError { message: "\"uid\"不一致".to_string() }.into());
    }
    // TODO
    let i = RB.update_by_id("", &user.0)
        .await
        .map_err(|e| {
            error::ErrorInternalServerError(e)
        })?;


    Ok(HttpResponse::Ok().body(i.to_string()))
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    println!("{}", req_body);
    HttpResponse::Ok().body(req_body)
}


// 请求必须带有 name 的 query 参数才会正确响应
#[get("/info")]
pub async fn get_info(info: web::Query<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}", info.name))
}

// 要返回不同类型可以使用 Either
pub async fn get_api_info(req: HttpRequest) -> Either<impl Responder, impl Responder> {
    let is_not_http_11 = req.head().version != Version::HTTP_11;
    if is_not_http_11 {
        return Either::A(HttpResponse::BadRequest().finish());
    }
    Either::B(HttpResponse::Ok().body("这是 v1 版本的 api！"))
}


// 必须有路由参数 user_id 和 friend 且类型正确才会正确响应
// 多个路由参数使用元组类型
#[get("/user/{user_id}/{friend}")]
pub async fn get_user_info(web::Path((user_id, friend)): web::Path<(u32, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome {}, user_id {}!", friend, user_id))
}


// 将request body中的信息反序列化到 Info 结构体中去
// 请求必须带有 Info 结构体类型的数据才会正确响应
#[get("/user/data")]
pub async fn user_data(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome {}!", info.name))
}


/// 使用 serde 提取表单数据
/// 仅当 content type 类型是  *x-www-form-urlencoded* 是 handler 处理函数才会被调用
/// 且请求中的内容能够被反序列化到一个 "Info" 结构体中去.
#[post("/user")]
pub async fn form(form: web::Form<Info>) -> Result<String, ()> {
    Ok(format!("Welcome {}!", form.name))
}


#[post("/payload")]
pub async fn payload(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    Ok(HttpResponse::Ok().body(format!("Body {}!", String::from_utf8(bytes.to_vec()).unwrap())))
}

#[get("/ee")]
async fn ee() -> Result<String, CustomError> {
    let error = CustomError::ValidationError { message: String::from("啦啦啦啦") };
    let result = Err(error);

    Ok(result.map_err(|e| e)?)
}

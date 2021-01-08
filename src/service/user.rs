use actix_web::{Either, Error, get, HttpRequest, HttpResponse, post, Responder, web};
use actix_web::http::Version;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Info {
    name: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Actix-Web!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    println!("{}", req_body);
    HttpResponse::Ok().body(req_body)
}


// 请求必须带有 name 的 query 参数才会正确响应
pub async fn manual_hello(info: web::Query<Info>) -> impl Responder {
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


#[get("/user/{user_id}")]
pub async fn get_user(web::Path(user_id): web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome, user_id {}!", user_id))
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

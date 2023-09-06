use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// 定义用户数据结构
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
}

// 用 Arc 和 Mutex 来创建一个线程安全的用户存储
type Users = Arc<Mutex<Vec<User>>>;

// 获取所有用户的处理函数
#[get("/users")]
async fn get_users(users: web::Data<Users>) -> impl Responder {
    let users = users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

// 创建新用户的处理函数
#[post("/users")]
async fn create_user(user: web::Json<User>, users: web::Data<Users>) -> impl Responder {
    let mut users = users.lock().unwrap();
    users.push(user.into_inner());
    HttpResponse::Created()
}

#[actix_web::main]
async fn main() ->  std::io::Result<()> {
    // 创建用户存储并包装在 Arc 和 Mutex 中
    let users: Users = Arc::new(Mutex::new(Vec::new()));

    // 启动 HTTP 服务器
    HttpServer::new(move || {
        App::new()
            .data(users.clone()) // 将用户存储传递给应用程序
            .service(get_users)
            .service(create_user)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


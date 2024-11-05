use std::{env, io, sync::Mutex};

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use errors::MyError;
use routers::{courses_routes, general_routes, teacher_routes};
use sqlx::mysql::MySqlPoolOptions;
use state::AppState;

#[path = "../db_access/mod.rs"]
mod db_access;
#[path = "../errors.rs"]
mod errors;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routes.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    // 如果提示环境变量没找到就进入到 web_service 目录再运行cargo run
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 中设置");

    let db_pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(Vec::new()),
        db: db_pool,
    });
    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::InvalidInput("Invalid input".to_string()).into()
            }))
            .wrap(cors)
            .configure(general_routes)
            .configure(courses_routes)
            .configure(teacher_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

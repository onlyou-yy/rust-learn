use std::{env, io, sync::Mutex};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routers::{courses_routes, general_routes};
use sqlx::mysql::MySqlPoolOptions;
use state::AppState;

#[path = "../db_access/mod.rs"]
mod db_access;
#[path = "../error.rs"]
mod error;
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
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(courses_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

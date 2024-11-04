use std::{io, sync::Mutex};

use actix_web::{web, App, HttpServer};
use routers::{courses_routes, general_routes};
use state::AppState;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(Vec::new()),
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(courses_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

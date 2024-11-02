use actix_web::{web, HttpResponse, Responder};

use crate::state::AppState;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> impl Responder {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

use crate::{api::HealthCheckResponse, state::AppState};
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let mut visit_count = app_state.visit_count.lock().unwrap();
    *visit_count += 1;

    HttpResponse::Ok().json(HealthCheckResponse {
        message: app_state.health_check_response.clone(),
        health_check_count: *visit_count,
    })
}

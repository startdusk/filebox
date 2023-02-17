use crate::handlers::{
    filebox::{add_new_filebox, get_filebox_by_code, take_filebox_by_code},
    general::health_check_handler,
};

use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn filebox_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/filebox", web::post().to(add_new_filebox));
    cfg.service(
        web::resource("/v1/filebox/{code}")
            .route(web::get().to(get_filebox_by_code))
            .route(web::post().to(take_filebox_by_code)),
    );
}

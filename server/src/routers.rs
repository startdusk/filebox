use crate::handlers::{
    filebox::{add_new_filebox, get_filebox_by_code},
    general::*,
};

use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn filebox_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/filebox/{code}", web::get().to(get_filebox_by_code));
    cfg.route("/v1/filebox", web::put().to(add_new_filebox));
}

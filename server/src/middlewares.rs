use crate::{
    data::redis::{
        add_ip_upload_limit_count, add_ip_visit_error_limit_count, is_allow_ip_for_upload,
        is_allow_ip_for_visit,
    },
    errors,
    state::CacheState,
};
use actix_http::body::BoxBody;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, Error,
};

use actix_web_lab::middleware::Next;

pub async fn ip_visit_error_limit_of_day_mw(
    cache_state: web::Data<CacheState>,
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let ip = get_ip(&req);

    let ip_allower = &cache_state.ip_allower;
    let addr = &cache_state.redis_actor;
    if !is_allow_ip_for_visit(addr, &ip, ip_allower.visit_error_limit).await? {
        return Ok(ServiceResponse::new(
            req.request().clone(),
            errors::Error::IpVisitErrorLimit(ip_allower.visit_error_limit).to_response(),
        ));
    }

    let res = next.call(req).await?;
    if res.response().error().is_some() {
        add_ip_visit_error_limit_count(addr, &ip, ip_allower.ttl).await?
    }

    Ok(res)
}

pub async fn ip_upload_limit_of_day_mw(
    cache_state: web::Data<CacheState>,
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let ip = get_ip(&req);

    let ip_allower = &cache_state.ip_allower;
    let addr = &cache_state.redis_actor;
    if !is_allow_ip_for_upload(addr, &ip, ip_allower.upload_limit).await? {
        return Ok(ServiceResponse::new(
            req.request().clone(),
            errors::Error::IpUploadLimit(ip_allower.upload_limit).to_response(),
        ));
    }

    add_ip_upload_limit_count(addr, &ip, ip_allower.ttl).await?;

    Ok(next.call(req).await?)
}

fn get_ip(req: &ServiceRequest) -> String {
    match req.headers().get("X-REAL-IP") {
        Some(header) => String::from(header.to_str().unwrap()),
        None => req.peer_addr().unwrap().ip().to_string(),
    }
}

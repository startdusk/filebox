use crate::{
    api::{DATE_FORMAT, IP_UPLOAD_LIMIT_HEADER, IP_VISIT_ERROR_LIMIT_HEADER},
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
use chrono::{DateTime, Datelike, Local, TimeZone, Utc};

pub async fn ip_visit_error_limit_of_day_mw(
    cache_state: web::Data<CacheState>,
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let ip_allower = &cache_state.ip_allower;
    let addr = &cache_state.redis_actor;

    if !is_allow_ip_from_header(&req, IP_VISIT_ERROR_LIMIT_HEADER) {
        return Ok(ServiceResponse::new(
            req.request().clone(),
            errors::Error::IpVisitErrorLimit(ip_allower.visit_error_limit).to_response(),
        ));
    }

    let ip = get_ip(&req);
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
    let ip_allower = &cache_state.ip_allower;
    let addr = &cache_state.redis_actor;

    if !is_allow_ip_from_header(&req, IP_UPLOAD_LIMIT_HEADER) {
        return Ok(ServiceResponse::new(
            req.request().clone(),
            errors::Error::IpUploadLimit(ip_allower.upload_limit).to_response(),
        ));
    }

    let ip = get_ip(&req);

    if !is_allow_ip_for_upload(addr, &ip, ip_allower.upload_limit).await? {
        return Ok(ServiceResponse::new(
            req.request().clone(),
            errors::Error::IpUploadLimit(ip_allower.upload_limit).to_response(),
        ));
    }

    add_ip_upload_limit_count(addr, &ip, ip_allower.ttl).await?;

    next.call(req).await
}

fn is_allow_ip_from_header(req: &ServiceRequest, header_name: &str) -> bool {
    let header_value = match req.headers().get(header_name) {
        Some(header) => header.to_str().unwrap(),
        None => return true,
    };

    match DateTime::parse_from_str(header_value, DATE_FORMAT) {
        Ok(t) => is_the_interval_one_day(t.timestamp()),
        Err(_) => false,
    }
}

fn is_the_interval_one_day(timestamp: i64) -> bool {
    let now = Local::now();
    let now = Utc
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
        .unwrap();

    (now.timestamp() - timestamp) >= 24 * 60 * 60
}

fn get_ip(req: &ServiceRequest) -> String {
    match req.headers().get("X-REAL-IP") {
        Some(header) => String::from(header.to_str().unwrap()),
        None => req.peer_addr().unwrap().ip().to_string(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_the_interval_one_day_should_work() {
        let now = Local::now();
        let prev_day = Utc
            .with_ymd_and_hms(now.year(), now.month(), now.day() - 1, 0, 0, 0)
            .unwrap();
        assert!(is_the_interval_one_day(prev_day.timestamp()));

        let next_day = Utc
            .with_ymd_and_hms(now.year(), now.month(), now.day() + 1, 0, 0, 0)
            .unwrap();
        assert!(!is_the_interval_one_day(next_day.timestamp()));
    }
}

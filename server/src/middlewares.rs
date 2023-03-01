use crate::{
    data::redis::{add_ip, allow_ip},
    errors,
    state::CacheState,
};
use actix_http::body::BoxBody;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, Error,
};

use actix_web_lab::middleware::Next;

pub async fn redis_ip_allower_mw(
    cache_state: web::Data<CacheState>,
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let peer_addr_ip = req.peer_addr().unwrap().ip().to_string();
    let ip = match req.headers().get("X-REAL-IP") {
        Some(header) => String::from(header.to_str().unwrap()),
        None => peer_addr_ip,
    };

    let ip_allower = &cache_state.ip_allower;
    let addr = &cache_state.redis_actor;
    if !allow_ip(addr, &ip, ip_allower.limit).await? {
        return Ok(ServiceResponse::new(
            req.request().clone(),
            errors::Error::IpAllowerError(ip_allower.limit).to_response(),
        ));
    }

    let res = next.call(req).await?;
    if res.response().error().is_some() {
        add_ip(addr, &ip, ip_allower.ttl).await?
    }

    Ok(res)
}

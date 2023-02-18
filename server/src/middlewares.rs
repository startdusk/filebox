use crate::{
    data::redis::{add_ip, allow_ip},
    errors,
    state::CacheState,
};
use actix_http::body::MessageBody;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, Error,
};

use actix_web_lab::middleware::Next;

pub async fn redis_ip_allower_mw(
    cache_state: web::Data<CacheState>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let peer_addr_ip = req.peer_addr().unwrap().ip().to_string();
    let ip = match req.headers().get("X-REAL-IP") {
        Some(header) => String::from(header.to_str().unwrap()),
        None => peer_addr_ip,
    };
    {
        let ip_allower = cache_state.ip_allower.lock().await;
        let mut ip_allower = ip_allower.borrow_mut();
        let limit = ip_allower.limit;
        if !allow_ip(&mut ip_allower.conn, ip.clone(), limit) {
            return Err(errors::Error::IpAllowerError(format!(
                "今日文件口令错误已达{}次, 请明天再访问",
                limit
            ))
            .into());
        }
        drop(ip_allower);
    }
    let res = next.call(req).await?;
    if res.response().error().is_some() {
        let ip_allower = cache_state.ip_allower.lock().await;
        let mut ip_allower = ip_allower.borrow_mut();
        let duration_day = ip_allower.duration_day;
        add_ip(&mut ip_allower.conn, ip, duration_day);
    }
    Ok(res)
}

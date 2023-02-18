use crate::{errors, state::FileboxState};
use actix_http::body::MessageBody;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, Error,
};

use actix_web_lab::middleware::Next;

pub async fn redis_ip_allower_mw(
    filebox_data: web::Data<FileboxState>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // TODO: lol
    let peer_addr_ip = req.peer_addr().unwrap().ip().to_string();
    let ip = match req.headers().get("X-REAL-IP") {
        Some(header) => String::from(header.to_str().unwrap()),
        None => peer_addr_ip,
    };
    let ip_allower = filebox_data.ip_allower.lock().await;
    {
        let mut ip_allower = ip_allower.borrow_mut();
        if !ip_allower.allow_ip(ip.clone()) {
            return Err(errors::Error::IpAllowerError(format!(
                "今日口令错误已达{}次, 请明天再访问",
                ip_allower.limit
            ))
            .into());
        }
    }
    let res = next.call(req).await?;
    if res.response().error().is_some() {
        let mut ip_allower = ip_allower.borrow_mut();
        ip_allower.add_ip(ip);
    }
    Ok(res)
}

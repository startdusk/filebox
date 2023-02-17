use std::{future::Future, ops::Add, pin::Pin, rc::Rc, sync::Arc};

use actix_http::{body::EitherBody, StatusCode};
use actix_utils::future::{ok, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use atomic_refcell::AtomicRefCell;
use chrono::{Duration, Local, NaiveDateTime};

use crate::api::{IPInfo, IPMap};

type Store = Arc<AtomicRefCell<IPAllowStore>>;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Debug)]
pub struct IPAllower {
    store: Store,
}

impl IPAllower {
    pub fn new(limit: i32, duration: Duration, ips: IPMap) -> Self {
        Self {
            store: Arc::new(AtomicRefCell::new(IPAllowStore::new(limit, duration, ips))),
        }
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for IPAllower
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = IPAllowMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(IPAllowMiddleware {
            service: Rc::new(service),
            store: self.store.clone(),
        })
    }
}

pub struct IPAllowMiddleware<S> {
    service: Rc<S>,
    store: Store,
}

impl<S, B> Service<ServiceRequest> for IPAllowMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let peer_addr_ip = req.peer_addr().unwrap().ip().to_string();
        let ip = match req.headers().get("X-REAL-IP") {
            Some(header) => String::from(header.to_str().unwrap()),
            None => peer_addr_ip,
        };

        let store = Arc::clone(&self.store);

        let srv = Rc::clone(&self.service);
        Box::pin(async move {
            let ip_allow = store.borrow();
            match ip_allow.allow_ip(ip.clone()) {
                true => srv.call(req).await.map(|err| {
                    // TODO: 判断路径和错误类型, 限制错误5次当日就不能再访问了
                    let mut ip_allow = store.borrow_mut();
                    ip_allow.add_ip(ip);
                    let status_code = err.status();
                    err.into_response(HttpResponse::new(status_code).map_into_right_body())
                }),
                false => Ok(req
                    .into_response(HttpResponse::new(StatusCode::FORBIDDEN).map_into_right_body())),
            }
        })
    }
}

#[derive(Debug)]
pub struct IPAllowStore {
    limit: i32,
    duration: Duration,
    ips: IPMap,
}

impl IPAllowStore {
    pub fn new(limit: i32, duration: Duration, ips: IPMap) -> Self {
        Self {
            limit,
            duration,
            ips,
        }
    }

    pub fn allow_ip(&self, ip: String) -> bool {
        match self.ips.get(&ip) {
            Some(ip_info) => {
                let ip_info = ip_info.borrow();
                if ip_info.count >= self.limit {
                    return false;
                }
                true
            }
            None => false,
        }
    }

    pub fn add_ip(&mut self, ip: String) {
        let now = self.now();
        if let dashmap::mapref::entry::Entry::Vacant(e) = self.ips.entry(ip.clone()) {
            let ip_info = IPInfo::new(1, now.add(self.duration).timestamp());
            e.insert(AtomicRefCell::new(ip_info));
        } else {
            let ip_info = self.ips.get(&ip).unwrap();
            let mut ip_info = ip_info.borrow_mut();
            ip_info.count += 1;
            ip_info.expired_at = now.add(self.duration).timestamp();
        }
    }

    fn now(&self) -> NaiveDateTime {
        Local::now().naive_local()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[tokio::test]
    // async fn ip_allow_should_work() {
    //     let mut ip_allow = IPAllowInner::new(1, chrono::Duration::seconds(1));

    //     let ip = "127.0.0.1".to_string();
    //     assert!(ip_allow.allow_ip(ip.clone()));
    //     ip_allow.add_ip(ip.clone());
    //     assert!(ip_allow.allow_ip(ip.clone()));
    //     // std::thread::sleep(std::time::Duration::new(1, 0));
    //     assert_eq!(ip_allow.allow_ip(ip.clone()), false);
    // }
}

use std::{
    cell::RefCell,
    collections::HashMap,
    future::{ready, Ready},
    ops::Add,
    rc::Rc,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use chrono::{Duration, Local, NaiveDateTime};
use futures_util::future::LocalBoxFuture;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Debug)]
pub struct IPAllower {
    pub limit: i32,
    pub duration: Duration,
}

impl IPAllower {
    pub fn new(limit: i32, duration: Duration) -> Self {
        Self { limit, duration }
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
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = IPAllowMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(IPAllowMiddleware {
            service: Rc::new(service),
            inner: Rc::new(tokio::sync::Mutex::new(RefCell::new(IPAllowInner::new(
                self.limit,
                self.duration,
            )))),
        }))
    }
}

pub struct IPAllowMiddleware<S> {
    service: Rc<S>,
    inner: Rc<tokio::sync::Mutex<RefCell<IPAllowInner>>>,
}

#[derive(Debug)]
pub struct IPInfo {
    pub count: i32,
    pub expired_at: i64,
}

impl<S, B> Service<ServiceRequest> for IPAllowMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let peer_addr_ip = req.peer_addr().unwrap().ip().to_string();
        let ip = match req.headers().get("X-REAL-IP") {
            Some(header) => String::from(header.to_str().unwrap()),
            None => peer_addr_ip,
        };

        let inner = self.inner.clone();

        let srv = self.service.clone();
        Box::pin(async move {
            let fut_inner = inner.clone();
            {
                let inner = inner.lock().await;
                let ip_allow = inner.borrow();
                if !ip_allow.allow_ip(ip.clone()) {
                    // let res = req.into_response(HttpResponse::Forbidden().finish());
                    // return Box::pin(async move { Ok(res) });
                }
            }

            let fut = srv.call(req);
            match fut.await {
                Ok(res) => Ok(res),
                Err(err) => {
                    let inner = fut_inner.lock().await;
                    let mut ip_allow = inner.borrow_mut();
                    ip_allow.add_ip(ip);
                    Err(err)
                }
            }
        })
    }
}

#[derive(Debug)]
pub struct IPAllowInner {
    pub limit: i32,
    pub duration: Duration,
    ips: HashMap<String, RefCell<IPInfo>>,
}

impl IPAllowInner {
    pub fn new(limit: i32, duration: Duration) -> Self {
        Self {
            limit,
            duration,
            ips: HashMap::new(),
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
        if let std::collections::hash_map::Entry::Vacant(e) = self.ips.entry(ip.clone()) {
            let ip_info = IPInfo {
                count: 1,
                expired_at: now.add(self.duration).timestamp(),
            };
            e.insert(RefCell::new(ip_info));
        } else {
            let ip_info = self.ips.get(&ip).unwrap();
            let mut ip_info = ip_info.borrow_mut();
            ip_info.count += 1;
            ip_info.expired_at = now.add(self.duration).timestamp();
        }
    }

    pub fn remove_expired_ip(&mut self) {
        let now_timestamp = self.now_timestamp();
        self.ips
            .retain(|_key, ip_info| ip_info.borrow().expired_at <= now_timestamp)
    }

    fn now(&self) -> NaiveDateTime {
        Local::now().naive_local()
    }

    fn now_timestamp(&self) -> i64 {
        self.now().timestamp()
    }
}

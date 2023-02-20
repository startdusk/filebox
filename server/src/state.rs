use sqlx::postgres::PgPool;
use std::{cell::RefCell, sync::Arc};
use tiny_id::ShortCodeGenerator;

use crate::{api::RedisActorAddr, data::redis::IpAllower};

#[derive(Debug)]
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: std::sync::Mutex<u64>,
    pub upload_path: String,
    pub db: PgPool,

    // 由于会 标准库中的 Mutex 在 .await中 会: this `MutexGuard` is held across an `await` point
    // 所以改用 tokio 的 Mutex
    pub code_gen: tokio::sync::Mutex<RefCell<ShortCodeGenerator<char>>>,
}

pub struct CacheState {
    pub ip_allower: Arc<IpAllower>,
    pub redis_actor: Arc<RedisActorAddr>,
}

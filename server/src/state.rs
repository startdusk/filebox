use sqlx::postgres::PgPool;
use std::cell::RefCell;
use tiny_id::ShortCodeGenerator;

#[derive(Debug)]
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: std::sync::Mutex<u64>,
    pub upload_path: String,
    pub db: PgPool,

    // 由于会在 标准库中的 Mutext 会: this `MutexGuard` is held across an `await` point
    // 所以改用 tokio 的 Mutex
    pub code_gen: tokio::sync::Mutex<RefCell<ShortCodeGenerator<char>>>,
}

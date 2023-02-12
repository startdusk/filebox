use sqlx::postgres::PgPool;
use std::{cell::RefCell, sync::Mutex};
use tiny_id::ShortCodeGenerator;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u64>,
    pub upload_path: String,
    pub db: PgPool,
    pub code_gen: Mutex<RefCell<ShortCodeGenerator<char>>>,
}

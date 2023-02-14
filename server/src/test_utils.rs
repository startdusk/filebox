use std::{cell::RefCell, path::Path};

use actix_web::{
    dev::{Service, ServiceResponse},
    test,
    web::{self, ServiceConfig},
    App,
};
use sqlx::PgPool;
use sqlx_db_tester::TestPg;
use tiny_id::ShortCodeGenerator;

use crate::state::AppState;

// private none test functions
pub fn get_tdb() -> TestPg {
    dotenvy::from_filename(".env.test").ok();
    let server_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let migrations = Path::new("./migrations");
    TestPg::new(server_url, migrations)
}

pub async fn create_test_app<F>(
    db_pool: &PgPool,
    f: F,
) -> impl Service<actix_http::Request, Response = ServiceResponse, Error = actix_web::Error>
where
    F: FnOnce(&mut ServiceConfig),
{
    let length: usize = 5;

    let generator = ShortCodeGenerator::new_lowercase_alphanumeric(length);

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: std::sync::Mutex::new(0),
        upload_path: "./todo".to_string(),
        db: db_pool.clone(),
        code_gen: tokio::sync::Mutex::new(RefCell::new(generator)),
    });
    test::init_service(App::new().app_data(shared_data.clone()).configure(f)).await
}

use std::env;
use std::{io, sync::Mutex};

use actix_web::{web, App, HttpServer};
use server::routers::{filebox_routes, general_routes};
use server::state::AppState;
use sqlx::postgres::PgPoolOptions;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(filebox_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:8888")?.run().await
}

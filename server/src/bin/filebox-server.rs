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
    let http_server_addr = env::var("HTTP_SERVER_ADDR").expect("HTTP_SERVER_ADDR is required");
    let upload_path = env::var("UPLOAD_FILE_PATH").expect("UPLOAD_FILE_PATH is required");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        upload_path,
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(filebox_routes)
    };

    println!("filebox server run on: {http_server_addr}");
    HttpServer::new(app).bind(http_server_addr)?.run().await
}

use std::cell::RefCell;
use std::env;
use std::{io, sync::Mutex};

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use server::routers::{filebox_routes, general_routes};
use server::scheduler::start_clean_expired_filebox;
use server::state::AppState;
use sqlx::postgres::PgPoolOptions;
use tiny_id::ShortCodeGenerator;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let http_server_addr = env::var("HTTP_SERVER_ADDR").expect("HTTP_SERVER_ADDR is required");
    let upload_path = env::var("UPLOAD_FILE_PATH").expect("UPLOAD_FILE_PATH is required");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    // The length of generated codes
    let length: usize = 5;

    // Create a generator. The generator must be mutable, because each
    // code generated updates its state.
    let generator = ShortCodeGenerator::new_lowercase_alphanumeric(length);

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        upload_path,
        db: db_pool.clone(),
        code_gen: Mutex::new(RefCell::new(generator)),
    });

    // Start scheduler on a new thread
    actix_rt::spawn(async move { start_clean_expired_filebox(&db_pool.clone()).await });

    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://127.0.0.1:5173")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(filebox_routes)
            .wrap(cors)
            .wrap(Logger::default())
    };

    log::info!("Filebox server run on: {http_server_addr}");
    HttpServer::new(app).bind(http_server_addr)?.run().await
}

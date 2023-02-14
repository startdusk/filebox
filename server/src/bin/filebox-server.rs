use std::cell::RefCell;
use std::env;
use std::io;
use std::time::Duration;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use server::routers::{filebox_routes, general_routes};
use server::scheduler::start_clean_expired_filebox;
use server::state::AppState;
use sqlx::postgres::PgPoolOptions;
use tiny_id::ShortCodeGenerator;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let http_server_addr = env::var("HTTP_SERVER_ADDR").expect("HTTP_SERVER_ADDR is required");
    let upload_path = env::var("UPLOAD_FILE_PATH").expect("UPLOAD_FILE_PATH is required");
    std::fs::create_dir_all(upload_path.clone())?;

    let graceful_shutdown_timeout_sec = env::var("GRACEFUL_SHUTDOWN_TIMEOUT_SEC")
        .expect("GRACEFUL_SHUTDOWN_TIMEOUT_SEC is required");
    let graceful_shutdown_timeout_sec: u64 = graceful_shutdown_timeout_sec.parse()
        .unwrap_or_else(|_| panic!("GRACEFUL_SHUTDOWN_TIMEOUT_SEC should be a u64 type but got {graceful_shutdown_timeout_sec}"));
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let length: usize = 5;
    let generator = ShortCodeGenerator::new_lowercase_alphanumeric(length);

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: std::sync::Mutex::new(0),
        upload_path: upload_path.clone(),
        db: db_pool.clone(),
        code_gen: tokio::sync::Mutex::new(RefCell::new(generator)),
    });

    // Start scheduler on a new thread
    let scheduler_handle = tokio::spawn(async move {
        start_clean_expired_filebox(&db_pool.clone(), upload_path.clone()).await
    });

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
    let server = HttpServer::new(app)
        .bind(http_server_addr)?
        .disable_signals()
        .shutdown_timeout(graceful_shutdown_timeout_sec) // Modified shutdown timeout, less than 30 seconds.
        .run();

    let server_handle = server.handle();
    let signal = tokio::signal::ctrl_c();

    tokio::pin!(server);
    tokio::select! {
        r = signal => {
            log::info!("received interrupt signal");
            r.unwrap();
            let ((), r) = tokio::join!(server_handle.stop(true), server);
            r.unwrap();

            tokio::time::sleep(Duration::from_secs(5)).await;
            scheduler_handle.abort();
        }
        r = &mut server => {
            log::info!("server finished");
            r.unwrap();
            tokio::time::sleep(Duration::from_secs(5)).await;
            scheduler_handle.abort();
        }
    }

    Ok(())
}

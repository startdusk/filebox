use std::cell::RefCell;
use std::env;
use std::io::Write;

use actix_cors::Cors;
use actix_web::middleware;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use chrono::Local;
use server::handlers::filebox::add_new_filebox;
use server::handlers::filebox::get_filebox_by_code;
use server::handlers::filebox::take_filebox_by_code;
use server::handlers::general::health_check_handler;
use server::middlewares::redis_ip_allower_mw;
use server::scheduler::start_clean_expired_filebox;
use server::state::AppState;
use server::state::FileboxState;
use server::IPAllower;
use sqlx::postgres::PgPoolOptions;
use tiny_id::ShortCodeGenerator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let env = env_logger::Env::new().default_filter_or("info");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            let level = { buf.default_styled_level(record.level()) };
            writeln!(
                buf,
                "[{}] {} [{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                record.module_path().unwrap_or("<unnamed>"),
                record.line().unwrap_or(0),
                &record.args(),
            )
        })
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let http_server_addr = env::var("HTTP_SERVER_ADDR").expect("HTTP_SERVER_ADDR is required");
    let upload_path = env::var("UPLOAD_FILE_PATH").expect("UPLOAD_FILE_PATH is required");
    std::fs::create_dir_all(upload_path.clone())?;

    let graceful_shutdown_timeout_sec = env::var("GRACEFUL_SHUTDOWN_TIMEOUT_SEC")
        .expect("GRACEFUL_SHUTDOWN_TIMEOUT_SEC is required");
    let graceful_shutdown_timeout_sec: u64 = graceful_shutdown_timeout_sec.parse()
        .unwrap_or_else(|_| panic!("GRACEFUL_SHUTDOWN_TIMEOUT_SEC should be a u64 type but got {graceful_shutdown_timeout_sec}"));
    let db_pool = PgPoolOptions::new().connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    let generator = ShortCodeGenerator::new_lowercase_alphanumeric(5);

    let redis_conn_addr = env::var("REDIS_CONN_ADDR").expect("REDIS_CONN_ADDR is required");
    let client = redis::Client::open(redis_conn_addr)?;
    let con = client.get_connection()?;

    let ip_allower = IPAllower::new(con, 5, 1);
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: std::sync::Mutex::new(0),
        upload_path: upload_path.clone(),
        db: db_pool.clone(),
        code_gen: tokio::sync::Mutex::new(RefCell::new(generator)),
    });

    let filebox_data = web::Data::new(FileboxState {
        ip_allower: tokio::sync::Mutex::new(RefCell::new(ip_allower)),
    });

    let pool = db_pool.clone();
    let scheduler_handle =
        tokio::spawn(async move { start_clean_expired_filebox(&pool, upload_path.clone()).await });

    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://127.0.0.1:5173")
            // .allowed_origin_fn(|origin, _req_head| {
            //     origin.as_bytes().starts_with(b"http://localhost")
            // })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(shared_data.clone())
            .app_data(filebox_data.clone())
            .wrap(middleware::DefaultHeaders::new().add(("Filebox-Version", "0.1")))
            .wrap(cors)
            .wrap(Logger::default())
            .route("/health", web::get().to(health_check_handler))
            .service(
                web::scope("/v1/filebox")
                    .wrap(from_fn(redis_ip_allower_mw))
                    .route("/", web::post().to(add_new_filebox))
                    .service(
                        web::resource("/{code}")
                            .route(web::get().to(get_filebox_by_code))
                            .route(web::post().to(take_filebox_by_code)),
                    ),
            )
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
            scheduler_handle.abort();
        }
        r = &mut server => {
            log::info!("server finished");
            r.unwrap();
            scheduler_handle.abort();
        }
    }

    Ok(())
}

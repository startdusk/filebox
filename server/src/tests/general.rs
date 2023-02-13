#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{routers::general_routes, state::AppState, test_utils::get_tdb};
    use tiny_id::ShortCodeGenerator;

    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_health() {
        let tdb = get_tdb();
        let db_pool = tdb.get_pool().await;
        let length: usize = 5;

        // Create a generator. The generator must be mutable, because each
        // code generated updates its state.
        let generator = ShortCodeGenerator::new_lowercase_alphanumeric(length);

        let shared_data = web::Data::new(AppState {
            health_check_response: "I'm OK.".to_string(),
            visit_count: std::sync::Mutex::new(0),
            upload_path: "./todo".to_string(),
            db: db_pool.clone(),
            code_gen: tokio::sync::Mutex::new(RefCell::new(generator)),
        });
        let app = test::init_service(
            App::new()
                .app_data(shared_data.clone())
                .configure(general_routes),
        )
        .await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}

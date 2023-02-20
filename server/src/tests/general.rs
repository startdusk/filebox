#[cfg(test)]
mod tests {

    use crate::test_utils::{create_test_app, get_tdb};

    use actix_web::test;

    #[actix_web::test]
    async fn test_health() {
        let tdb = get_tdb();
        let db_pool = tdb.get_pool().await;
        let app = create_test_app(&db_pool).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}

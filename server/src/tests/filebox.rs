#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{routers::filebox_routes, state::AppState, test_utils::get_tdb};

    use actix_web::{dev::Service, http::header::ContentType, test, web, App};
    use serde::Serialize;
    use tiny_id::ShortCodeGenerator;

    #[derive(Debug, Serialize)]
    struct CreateFileboxForm {
        pub name: String,
        pub text: Option<String>,
        pub duration_day: u8,
        // pub file_type: FileboxFileType,
    }

    #[actix_web::test]
    async fn test_filebox_lifecycle() {
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
                .configure(filebox_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/v1/filebox")
            .insert_header(ContentType(mime::MULTIPART_FORM_DATA))
            .insert_header(ContentType::form_url_encoded())
            // .set_form(web::Form(CreateFileboxForm{
            //     name: "test".to_string(),
            //     text: Some("123456".to_string()),
            //     duration_day: 7,
            //     // file_type: FileboxFileType::Text,
            // }))
            .to_request();
        let resp = app.call(req).await.unwrap();
        dbg!(resp.response().body());
    }
}

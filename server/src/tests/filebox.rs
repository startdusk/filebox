#[cfg(test)]
mod tests {
    use std::ops::Add;

    use crate::{
        api::GetFileboxResponse,
        data::postgres::add_new_filebox_db,
        models::filebox::{AddFilebox, FileType},
        test_utils::{create_test_app, get_tdb},
    };

    use actix_web::test;
    use chrono::{Duration, Local};

    // #[derive(Debug, Serialize)]
    // struct CreateFileboxForm {
    //     pub name: String,
    //     pub text: String,
    //     pub duration_day: u8,
    //     pub file_type: FileboxFileType,
    // }

    #[actix_web::test]
    async fn test_filebox_lifecycle() {
        let tdb = get_tdb();
        let db_pool = tdb.get_pool().await;

        let app = create_test_app(&db_pool.clone()).await;

        // TODO: How can i send a multipart(file) to TestRequest? #2512: https://github.com/actix/actix-web/discussions/2512
        // let req = test::TestRequest::post()
        //     .uri("/v1/filebox")
        //     .insert_header(ContentType::json())
        //     .set_form(&CreateFileboxForm {
        //         name: "test".to_string(),
        //         text: "123456".to_string(),
        //         duration_day: 7,
        //         file_type: FileboxFileType::Text,
        //     })
        //     .to_request();
        // let resp = app.call(req).await.unwrap();

        // 曲线救国 lol
        let code = "12345".to_string();
        let now = Local::now().naive_local();
        let filebox = AddFilebox {
            code: code.clone(),
            name: "test".to_string(),
            file_type: FileType::Text,
            text: "21123".to_string(),
            created_at: now,
            expired_at: now.add(Duration::days(7)),
            ..Default::default()
        };
        let new_filebox = add_new_filebox_db(&db_pool, filebox.clone()).await.unwrap();
        assert_eq!(filebox.code, new_filebox.code);
        assert_eq!(filebox.name, new_filebox.name);
        assert_eq!(filebox.file_type, new_filebox.file_type);
        assert_eq!(filebox.text, new_filebox.text);
        assert_eq!(
            filebox.created_at.timestamp(),
            new_filebox.created_at.timestamp()
        );
        assert_eq!(
            filebox.expired_at.timestamp(),
            new_filebox.expired_at.timestamp()
        );

        let uri = &format!("/v1/filebox/{code}");
        let get_filebox_req = test::TestRequest::get().uri(uri).to_request();
        let get_filebox: GetFileboxResponse =
            test::call_and_read_body_json(&app, get_filebox_req).await;
        assert_eq!(get_filebox.code, new_filebox.code);
        assert_eq!(get_filebox.name, new_filebox.name);
        assert_eq!(get_filebox.created_at, new_filebox.created_at.timestamp());
        assert_eq!(get_filebox.expired_at, new_filebox.expired_at.timestamp());

        // let take_filebox_req = test::TestRequest::post().uri(uri).to_request();
        // let take_filebox: TakeTextResponse =
        //     test::call_and_read_body_json(&app, take_filebox_req).await;
        // assert!(take_filebox.used_at > 0);
    }
}

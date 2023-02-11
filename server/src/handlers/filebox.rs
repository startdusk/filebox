use std::fs;
use std::ops::Add;

use actix_easy_multipart::MultipartForm;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Local};
use uuid::Uuid;
use validator::Validate;

use crate::api::{
    CreateFileboxRequest, CreateFileboxResponse, FileboxFileType, GetFileboxResponse,
};
use crate::dbaccess::filebox::{add_new_filebox_db, get_filebox_db};
use crate::error::Error;
use crate::models::filebox::{AddFilebox, FileType};
use crate::state::AppState;

pub async fn get_filebox_by_code(
    app_state: web::Data<AppState>,
    code: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let code = code.into_inner();

    let filebox = get_filebox_db(&app_state.db, code).await?;
    let resp: GetFileboxResponse = filebox.into();
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn add_new_filebox(
    app_state: web::Data<AppState>,
    form: MultipartForm<CreateFileboxRequest>,
) -> Result<HttpResponse, Error> {
    // TODO: 生成5位数的提取码
    let code = "12345".to_string();

    let now = Local::now().naive_local();
    let form = form.into_inner(); // need to take mutable ownership of the form
    form.validate()?;
    let day = *form.duration_day as i64;
    let name = &*form.name;

    let file_type = *form.file_type;
    let new_filebox = match file_type {
        FileboxFileType::Text => {
            let text = &*form.text.unwrap();
            AddFilebox {
                code,
                name: name.clone(),
                file_type: FileType::Text,
                text: text.clone(),
                created_at: now,
                expired_at: now.add(Duration::days(day)),
                ..Default::default()
            }
        }
        FileboxFileType::File => {
            let prefix = format!("{}/{}", app_state.upload_path, Uuid::new_v4());
            fs::create_dir_all(prefix.clone())?;

            let upload_file = form.file.unwrap();
            let store_filepath = format!("{}/{}", prefix, upload_file.file_name.unwrap(),);
            upload_file.file.persist(store_filepath).unwrap();
            AddFilebox {
                code,
                name: name.clone(),
                file_type: FileType::File,
                created_at: now,
                expired_at: now.add(Duration::days(day)),
                ..Default::default()
            }
        }
    };

    let new_filebox = add_new_filebox_db(&app_state.db, new_filebox).await?;
    let resp: CreateFileboxResponse = new_filebox.into();
    Ok(HttpResponse::Ok().json(resp))
}

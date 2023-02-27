use std::fs;
use std::ops::Add;
use std::path::Path;

use actix_easy_multipart::MultipartForm;
use actix_files::NamedFile;
use actix_http::header::{Charset, ExtendedValue};
use actix_http::{body, header};
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use actix_web::web::Bytes;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Duration, Local};
use uuid::Uuid;
use validator::Validate;

use crate::api::{
    CreateFileboxRequest, CreateFileboxResponse, FileboxFileType, GetFileboxResponse,
    TakeTextResponse,
};
use crate::data::postgres::{add_new_filebox_db, get_filebox_db, update_filebox_db};
use crate::errors::Error;
use crate::models::filebox::{AddFilebox, FileType};
use crate::state::AppState;

pub async fn get_filebox_by_code(
    app_state: web::Data<AppState>,
    code: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let code = code.into_inner();

    let filebox = get_filebox_db(&app_state.db, code).await?;

    if filebox.has_taken() {
        return Ok(HttpResponse::BadRequest().body("file box has taken"));
    }
    let resp: GetFileboxResponse = filebox.into();
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn add_new_filebox(
    app_state: web::Data<AppState>,
    form: MultipartForm<CreateFileboxRequest>,
) -> Result<HttpResponse, Error> {
    let code_gen = app_state.code_gen.lock().await;
    let code = code_gen.borrow_mut().next_string();

    let form = form.into_inner(); // need to take mutable ownership of the form
    form.validate()?;
    let day = *form.duration_day as i64;
    let name = &*form.name;

    let file_type = *form.file_type;

    let now = Local::now().naive_local();
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
            let folder_name = Uuid::new_v4().to_string();
            let prefix = format!("{}/{}", app_state.upload_path, folder_name);
            fs::create_dir_all(prefix.clone())?;

            let upload_file = form.file.unwrap();
            let file_name = upload_file.file_name.unwrap();
            let store_filepath = format!("{prefix}/{file_name}");
            upload_file.file.persist(store_filepath).unwrap();
            AddFilebox {
                code,
                name: name.clone(),
                file_type: FileType::File,
                file_path: format!("{folder_name}/{file_name}"),
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

pub async fn take_filebox_by_code(
    app_state: web::Data<AppState>,
    code: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let code = code.into_inner();

    let filebox = update_filebox_db(&app_state.db, code).await?;
    match filebox.file_type {
        FileType::Text => {
            let resp: TakeTextResponse = filebox.into();
            let file_name = format!("{}.txt", resp.name);
            let cd = ContentDisposition {
                parameters: vec![DispositionParam::FilenameExt(ExtendedValue {
                    charset: Charset::Gb2312,
                    language_tag: None,
                    value: file_name.into(),
                })],
                disposition: DispositionType::Attachment,
            };
            let stream = body::BoxBody::new(Bytes::from(resp.text));

            let mut resp = HttpResponse::Ok();
            let resp = resp
                .append_header((header::CONTENT_DISPOSITION, cd))
                .append_header((header::ACCESS_CONTROL_EXPOSE_HEADERS, "Content-Disposition"))
                .message_body(stream)?;
            Ok(resp)
        }
        FileType::File => {
            let file_name = Path::new(&filebox.file_path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();

            let file_path = format!("{}/{}", app_state.upload_path, filebox.file_path);
            let file_stream = NamedFile::open_async(file_path).await?;
            let into_resp = file_stream.into_response(&req);
            let mut resp = HttpResponse::Ok();
            let cd = ContentDisposition {
                parameters: vec![DispositionParam::FilenameExt(ExtendedValue {
                    charset: Charset::Gb2312,
                    language_tag: None,
                    value: file_name.into(),
                })],
                disposition: DispositionType::Attachment,
            };
            let resp = resp
                .append_header((header::CONTENT_DISPOSITION, cd))
                .append_header((header::ACCESS_CONTROL_EXPOSE_HEADERS, "Content-Disposition"))
                .message_body(into_resp.into_body())?;

            Ok(resp)
        }
    }
}

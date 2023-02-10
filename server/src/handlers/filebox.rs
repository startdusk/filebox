use std::io::Write;
use std::ops::Add;

use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Local};
use futures::{StreamExt, TryStreamExt};
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
    req: web::Json<CreateFileboxRequest>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    req.validate()?;

    // TODO: 生成5位数的提取码
    let code = "12345".to_string();
    let now = Local::now().naive_local();

    let name = req.name.clone();
    let day = req.durations_day as i64;
    let new_filebox = match req.file_type {
        FileboxFileType::Text => {
            let text = req.text.clone().unwrap();
            AddFilebox {
                code,
                name,
                file_type: FileType::Text,
                text,
                created_at: now,
                expired_at: now.add(Duration::days(day)),
                ..Default::default()
            }
        }
        FileboxFileType::File => {
            // iterate over multipart stream
            while let Ok(Some(mut field)) = payload.try_next().await {
                let content_type = field.content_disposition();
                let filename = content_type.get_filename().unwrap();
                let filepath = format!("./store/{}{}", Uuid::new_v4(), filename);

                // File::create is blocking operation, use threadpool
                let f = web::block(|| std::fs::File::create(filepath))
                    .await
                    .unwrap();
                let mut f = f.unwrap();
                // Field in turn is stream of *Bytes* object
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    // filesystem operations are blocking, we have to use threadpool
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .unwrap();
                }
            }
            AddFilebox {
                code,
                name,
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

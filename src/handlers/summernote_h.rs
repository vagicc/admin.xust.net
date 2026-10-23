use crate::session::Session;
use crate::template::view;
use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::Json;
use rand::Rng;
use serde_json::value::Map;
use std::collections::HashMap;

pub async fn demo_html(session: Session) -> Response {
    log::info!("输出修改推荐");
    let data = Map::new();
    let html = view("upload_demo.html", data, session);
    Html(html).into_response()
}

pub async fn upload_demo(_session: Session, mut form: Multipart) -> Response {
    let mut field_names = Vec::new();
    while let Ok(Some(field)) = form.next_field().await {
        let name = field.name().map(|s| s.to_string()).unwrap_or_default();
        let filename = field.file_name().map(|s| s.to_string()).unwrap_or_default();
        let data = field.bytes().await.unwrap_or_default();
        field_names.push((name, filename, String::from_utf8_lossy(&data).to_string()));
    }
    println!("上传的：{:#?}", field_names);
    Html("文件上传成功!".to_string()).into_response()
}

pub async fn upload_summernote(_session: Session, mut form: Multipart) -> Response {
    const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024;

    let mut image = None;
    loop {
        let field = match form.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(error) => {
                log::warn!("读取 Summernote 上传表单失败: {}", error);
                return upload_error(StatusCode::BAD_REQUEST, "无法读取上传文件");
            }
        };

        if field.name() != Some("file") {
            continue;
        }
        if image.is_some() {
            return upload_error(StatusCode::BAD_REQUEST, "一次只能上传一张图片");
        }
        if field.file_name().is_none() {
            return upload_error(StatusCode::BAD_REQUEST, "缺少 file 图片字段");
        }

        let mut bytes = Vec::new();
        let mut field = field;
        loop {
            match field.chunk().await {
                Ok(Some(chunk)) => {
                    if bytes.len() + chunk.len() > MAX_IMAGE_SIZE {
                        return upload_error(StatusCode::BAD_REQUEST, "图片超过 10 MB");
                    }
                    bytes.extend_from_slice(&chunk);
                }
                Ok(None) => break,
                Err(error) => {
                    log::warn!("读取 Summernote 上传内容失败: {}", error);
                    return upload_error(StatusCode::BAD_REQUEST, "无法读取上传文件");
                }
            }
        }
        image = Some(bytes);
    }

    let Some(bytes) = image else {
        return upload_error(StatusCode::BAD_REQUEST, "缺少 file 图片字段");
    };
    if bytes.is_empty() {
        return upload_error(StatusCode::BAD_REQUEST, "图片为空");
    }

    let Some(extension) = image_extension(&bytes) else {
        return upload_error(
            StatusCode::BAD_REQUEST,
            "仅支持 JPEG、PNG、GIF 和 WebP 图片",
        );
    };

    let now = chrono::Local::now();
    let year = now.format("%Y").to_string();
    let relative_dir = format!("static/upload/summernote/{}", year);
    if let Err(error) = tokio::fs::create_dir_all(&relative_dir).await {
        log::error!("创建 Summernote 图片目录失败: {}", error);
        return upload_error(StatusCode::INTERNAL_SERVER_ERROR, "图片保存失败");
    }

    let timestamp = now.format("%Y%m%d%H%M%S");
    let random_suffix = rand::thread_rng().gen_range(0..1000);
    let filename = format!("{}{:03}.{}", timestamp, random_suffix, extension);
    let path = std::path::Path::new(&relative_dir).join(&filename);
    let mut file = match tokio::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .await
    {
        Ok(file) => file,
        Err(error) => {
            log::error!("创建 Summernote 图片文件失败: {}", error);
            return upload_error(StatusCode::INTERNAL_SERVER_ERROR, "图片保存失败");
        }
    };
    if let Err(error) = tokio::io::AsyncWriteExt::write_all(&mut file, &bytes).await {
        log::error!("写入 Summernote 图片失败: {}", error);
        return upload_error(StatusCode::INTERNAL_SERVER_ERROR, "图片保存失败");
    }

    Json(serde_json::json!({
        "status": true,
        "image": format!("/static/upload/summernote/{}/{}", year, filename)
    }))
    .into_response()
}

fn upload_error(status: StatusCode, message: &str) -> Response {
    (
        status,
        Json(serde_json::json!({
            "status": false,
            "error": message
        })),
    )
        .into_response()
}

fn image_extension(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(b"\xFF\xD8\xFF") {
        Some("jpg")
    } else if bytes.starts_with(b"\x89PNG\r\n\x1A\n") {
        Some("png")
    } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        Some("gif")
    } else if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        Some("webp")
    } else {
        None
    }
}

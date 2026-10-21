use crate::session::Session;
use crate::template::view;
use axum::extract::Multipart;
use axum::response::{Html, IntoResponse, Response};
use bytes::BufMut;
use futures_util::TryStreamExt;
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

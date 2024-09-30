use crate::session::Session;
use crate::template::view;
use serde_json::value::Map;
use std::collections::HashMap;
use warp::{Rejection, Reply};

use bytes::BufMut;
use futures_util::TryStreamExt;
use warp::multipart::FormData;
use warp::Filter;

#[derive(Debug)]
struct ServerError {
    message: String,
}
impl warp::reject::Reject for ServerError {}

// 输出html
pub async fn demo_html(session: Session) -> Result<impl Reply, Rejection> {
    log::info!("输出修改推荐");

    let mut data = Map::new();

    let html = view("upload_demo.html", data, session);
    Ok(warp::reply::html(html))
}

pub async fn upload_demo(form: FormData, session: Session) -> Result<impl Reply, Rejection> {
    /* 处理文件上传表单（method="post" enctype="multipart/form-data"） */
    let field_names: Vec<_> = form
        .and_then(|mut field| async move {
            let mut bytes: Vec<u8> = Vec::new();

            // field.data() only returns a piece of the content, you should call over it until it replies None
            while let Some(content) = field.data().await {
                let content = content.unwrap();
                bytes.put(content);
            }
            print!("bytes:{:#?}",bytes);
            println!("kk:{:#?}", field);
            Ok((
                field.name().to_string(),
                field.filename().unwrap().to_string(),
                String::from_utf8_lossy(&*bytes).to_string(),
            ))
        })
        .try_collect()
        .await
        .unwrap();
    println!("上传的：{:#?}", field_names);

    let mut post: HashMap<String, String> = HashMap::new();

    println!("上传的表单：{:#?}", post);

    Ok("文件上传成功!")
}

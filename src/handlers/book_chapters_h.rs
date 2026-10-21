use crate::models::book_chapters_content_m;
use crate::models::book_chapters_m;
use crate::session::Session;
use crate::template::view;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use handlebars::to_json;
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;

//书籍章节列表
//响应GET: /book/chapters/list/{1}
pub async fn list_page(page: u32, get: book_chapters_m::GetQuery, session: Session) -> Response {
    log::debug!("书籍章节列表GET: /book/chapters/list");

    let (count, list, pages) = book_chapters_m::list_page(
        Some(page),
        Some(crate::constants::PER_PAGE),
        Some(get.clone()),
    );

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count));
    data.insert("list".to_string(), to_json(list));
    data.insert("pages".to_string(), to_json(pages));
    data.insert("get".to_string(), to_json(get));

    let html = view("chapters/list.html", data, session);
    Html(html).into_response()
}

pub async fn get_chapters_content(
    Path(chapter_id): Path<i32>,
    _session: Session,
) -> impl IntoResponse {
    let chapters_content = book_chapters_content_m::find_chapters_content(chapter_id);
    if chapters_content.is_none() {
        return crate::common::response_json(
            StatusCode::NO_CONTENT,
            None,
            Some("204查无数据".to_owned()),
        );
    }

    let data = chapters_content.unwrap();
    crate::common::response_json(
        StatusCode::OK,
        Some(&data),
        Some("取得文章章节正文成功".to_owned()),
    )
}

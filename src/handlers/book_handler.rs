use crate::models::book_chapters_m;
use crate::models::books_model;
use crate::session::Session;
use crate::template::view;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;

//书籍列表
//响应GET: /book/list/{1}
pub async fn list_page(page: u32, get: books_model::GetQuery, session: Session) -> Response {
    log::debug!("书籍列表GET: /book/list");

    let (count, list, pages) = books_model::list_page(
        Some(page),
        Some(crate::constants::PER_PAGE),
        Some(get.clone()),
    );

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count));
    data.insert("list".to_string(), to_json(list));
    data.insert("pages".to_string(), to_json(pages));
    data.insert("get".to_string(), to_json(get));

    let html = view("book/list.html", data, session);
    Html(html).into_response()
}

// 书籍章节
// 响应GET: /book/chapters/{book_id}
pub async fn chapters(Path(book_id): Path<i32>, session: Session) -> Response {
    let book = books_model::find_book(book_id);

    if book.is_none() {
        let html = "查无此书籍".to_string();
        return Html(html).into_response();
    }

    let chapters = book_chapters_m::get_book_all_chapters(book_id);
    let mut data = Map::new();
    data.insert("book".to_string(), to_json(book));
    data.insert("chapters".to_string(), to_json(chapters));

    let html = view("book/chapters.html", data, session);
    Html(html).into_response()
}

// 定义查询参数的结构体
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    name: Option<String>,
    age: Option<i32>,
}

pub async fn test(params: SearchParams) -> Response {
    println!("get参数：{:#?}", params);
    let html = format!("测试GET参数：{:#?}", params);
    Html(html).into_response()
}

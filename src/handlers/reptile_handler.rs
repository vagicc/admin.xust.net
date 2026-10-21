use crate::session::Session;
use axum::response::{Html, IntoResponse, Response};

pub async fn test_html_select(_: Session) -> Response {
    Html("这里用来测试HTML").into_response()
}

pub async fn test_zhonghuadiancang_detail(_: Session) -> Response {
    let url = "https://www.zhonghuadiancang.com/xuanxuewushu/18783/344485.html".to_string();
    zhonghuadiancang_book_chapter(url).await;
    Html("这里用来测试HTML").into_response()
}

async fn zhonghuadiancang_book_chapter(url: String) {
    let result = crate::http::http_request(&url).await;
    let response = result.unwrap();
    let html = response.html.as_str();
    println!("到这===");
    let _data = crate::parse::zhdc_book_chapter_select(html).await;
}

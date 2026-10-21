use crate::template::view;
use axum::response::{Html, IntoResponse, Response};
use handlebars::to_json;
use serde_json::value::Map;

pub async fn index(session: crate::session::Session) -> Response {
    log::warn!("[测试]：{:#?}", session);
    let data = Map::new();
    let html = view("index.html", data, session);
    Html(html).into_response()
}

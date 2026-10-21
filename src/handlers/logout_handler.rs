use crate::session;
use crate::template::to_html_single;
use axum::http::{header, Response as HttpResponse, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use handlebars::to_json;
use serde_json::value::Map;

pub async fn quit() -> Response {
    log::info!("logout:退出登录处理");

    let cookie = session::Cookie::new();
    log::warn!("Cookie:{:#?}", cookie);

    let mut data = Map::new();
    data.insert("jump_url".to_string(), to_json("/login"));
    data.insert("message".to_string(), to_json("退出登录成功!!"));
    let html = to_html_single("hint.html", data);

    HttpResponse::builder()
        .status(StatusCode::OK)
        .header(
            header::SET_COOKIE,
            format!(
                "{}={}; SameSite=Strict; Max-Age=0; HttpOnly",
                crate::constants::SESSION_NAME,
                serde_json::json!(cookie)
            ),
        )
        .body(html)
        .unwrap()
        .into_response()
}

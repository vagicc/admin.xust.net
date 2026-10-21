use crate::template::to_html_single;
use axum::extract::Form;
use axum::http::{header, StatusCode};
use axum::response::{Html, IntoResponse, Redirect, Response};
use handlebars::to_json;
use serde_json::value::Map;

pub async fn redirect_index() -> Response {
    log::debug!("[调试信息]访问了“/demo/redirect”");
    Html("重定向示例".to_string()).into_response()
}

pub async fn show_repeat_html() -> Response {
    let mut data = Map::new();
    data.insert(
        "base_url".to_string(),
        to_json(crate::common::get_env("BASE_URL")),
    );
    Html(to_html_single("demo_repeat.html", data)).into_response()
}

pub async fn repeat(post: u32) -> Response {
    log::debug!("[调试信息]访问了“/demo/redirect”");
    if post == 0 {
        log::warn!("无post数据，输出表单");
        return StatusCode::NOT_FOUND.into_response();
    }
    axum::http::Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/demo/redirect/v")
        .body(String::new())
        .unwrap()
        .into_response()
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RepeatForm {
    pub username: String,
}

impl RepeatForm {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.username.is_empty() {
            return Err("用户名不能为空");
        }
        Ok(self.clone())
    }
}

pub async fn repeat_post(Form(form): Form<RepeatForm>) -> Response {
    match form.validate() {
        Ok(post) => {
            println!("post:{:#?}", post);
        }
        Err(msg) => {
            println!("出错：{}", msg);
        }
    }
    axum::http::Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/demo/redirect/v")
        .body(String::new())
        .unwrap()
        .into_response()
}

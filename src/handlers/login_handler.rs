use crate::constants::SESSION_NAME;
use crate::models::admins_model;
use crate::session;
use crate::template::to_html_single;
use axum::http::{header, Response as HttpResponse, StatusCode, Uri};
use axum::response::{Html, IntoResponse, Redirect, Response};
use handlebars::to_json;
use serde::Deserialize;
use serde_json::value::Map;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub passwd: String,
}

impl LoginForm {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.username.is_empty() {
            Err("登录名不能为空")
        } else if self.passwd.is_empty() {
            Err("密码不能为空")
        } else {
            Ok(self.clone())
        }
    }
}

pub async fn show_login_html() -> Response {
    let mut data = Map::new();
    data.insert(
        "base_url".to_string(),
        to_json(crate::common::get_env("BASE_URL")),
    );
    Html(to_html_single("login.html", data)).into_response()
}

pub async fn check_login(form: LoginForm) -> Response {
    let mut data = Map::new();

    match form.validate() {
        Ok(post) => {
            let admin = admins_model::get_admin(post.username.clone());
            if admin.is_none() {
                log::warn!("查无用户");
                data.insert("jump_url".to_string(), to_json("/login"));
                data.insert("message".to_string(), to_json("查无用户"));
                let html = to_html_single("hint.html", data);
                return HttpResponse::builder()
                    .status(StatusCode::OK)
                    .body(html)
                    .unwrap()
                    .into_response();
            }

            let admin = admin.unwrap();
            let password = admins_model::encryption(&post.passwd, &admin.salt);
            if password != admin.password {
                log::warn!("密码{}出错:{} != {}", post.passwd, password, admin.password);
                data.insert("jump_url".to_string(), to_json("/login"));
                data.insert("message".to_string(), to_json("用户密码错误"));
                let html = to_html_single("hint.html", data);
                return HttpResponse::builder()
                    .status(StatusCode::OK)
                    .body(html)
                    .unwrap()
                    .into_response();
            }

            use crate::common::random_key;
            let cookie = session::Cookie {
                id: random_key(34),
                passwd: admin.password.clone(),
                timestamp: random_key(8),
                userid: admin.id,
            };

            let session_value = serde_json::json!(cookie);
            let cookie_encode = base64::encode(session_value.to_string().as_bytes());
            let cookie_value = format!(
                "{}={}; Path=/; SameSite=Lax; HttpOnly",
                SESSION_NAME, cookie_encode
            );

            data.insert("jump_url".to_string(), to_json("/"));
            data.insert("message".to_string(), to_json("登录成功"));
            let html = to_html_single("hint.html", data);

            return HttpResponse::builder()
                .status(StatusCode::OK)
                .header(header::SET_COOKIE, cookie_value)
                .body(html)
                .unwrap()
                .into_response();
        }
        Err(msg) => {
            log::error!("{}", msg);
            data.insert("jump_url".to_string(), to_json("/login"));
            data.insert("message".to_string(), to_json(msg));
            let html = to_html_single("hint.html", data);
            return HttpResponse::builder()
                .status(StatusCode::OK)
                .body(html)
                .unwrap()
                .into_response();
        }
    }
}

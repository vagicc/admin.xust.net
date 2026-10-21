use crate::handlers::login_handler;
use axum::extract::{ConnectInfo, Form, OriginalUri};
use axum::http::{HeaderMap, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, OnceLock};

static LOGIN_URL_COUNTERS: OnceLock<Arc<Mutex<HashMap<String, usize>>>> = OnceLock::new();

fn login_url_counters() -> Arc<Mutex<HashMap<String, usize>>> {
    LOGIN_URL_COUNTERS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
        .clone()
}

pub fn router() -> Router {
    let counters = login_url_counters();

    Router::new()
        .route("/login", get(login_handler::show_login_html))
        .route(
            "/login/check",
            post(|Form(form): Form<login_handler::LoginForm>| async move {
                login_handler::check_login(form).await
            }),
        )
        .route(
            "/login_url",
            get(move |OriginalUri(uri): OriginalUri| {
                let mut counts = counters.lock().unwrap();
                let count = *counts
                    .entry(uri.path().to_string())
                    .and_modify(|c| *c += 1)
                    .or_insert(0);
                async move { format!("This is the {}th visit to this URL!", count) }
            }),
        )
        .route(
            "/login_ip",
            get(|headers: HeaderMap| async move {
                let ip_header = headers
                    .get("x-real-ip")
                    .or_else(|| headers.get("x-forwarded-for"));

                if let Some(value) = ip_header {
                    if let Ok(ip_str) = value.to_str() {
                        return format!("ip: {}", ip_str);
                    }
                }

                "ip: unknown".to_string()
            }),
        )
        .route(
            "/remote_addr",
            get(|ConnectInfo(addr): ConnectInfo<SocketAddr>| async move {
                let ip = addr.ip();
                format!("用户IP: {}", ip)
            }),
        )
        .route(
            "/login_h",
            get(
                |headers: HeaderMap, OriginalUri(uri): OriginalUri| async move {
                    format!(
                        "header count: {:#?} <br><hr> fullpath请求的后缀:{:#?}",
                        headers,
                        uri.path()
                    )
                },
            ),
        )
}

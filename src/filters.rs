use crate::routes::{
    admins_route, article_category_r, article_route, book_route, chapters_route, check_route,
    column_route, demo_route, home_route, login_route, logout_route, menus_route, reptile_route,
    rights_route, role_route, upload_route,
};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    extract::Path,
    routing::{get, get_service},
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    format!("你好，{}!", name)
}

pub fn all_routes() -> Router {
    Router::new()
        .route(
            "/favicon.ico",
            get_service(ServeFile::new("./static/favicon.ico")),
        )
        .nest_service(
            "/.well-known",
            get_service(ServeDir::new("./static/.well-known")),
        )
        .nest_service("/static", get_service(ServeDir::new("./static")))
        .route("/hello/{name}", get(hello))
        .merge(login_route::router())
        .merge(menus_route::router())
        .merge(admins_route::router())
        .merge(role_route::router())
        .merge(rights_route::router())
        .merge(logout_route::router())
        .merge(home_route::router())
        .merge(demo_route::router())
        .merge(reptile_route::router())
        .merge(check_route::router())
        .merge(book_route::router())
        .merge(chapters_route::router())
        .merge(article_route::router())
        .merge(article_category_r::router())
        .merge(column_route::router())
        .merge(upload_route::router())
}

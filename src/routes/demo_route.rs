use crate::handlers::demo_handler;
use axum::http::{header, Uri};
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/demo/redirect", get(demo_handler::redirect_index))
        .route("/demo/redirect/v", get(|| async { "这里是V" }))
        .route(
            "/demo/redirect/v1",
            get(|| async { Redirect::permanent("/demo/redirect/v") }),
        )
        .route(
            "/demo/redirect/v8",
            get(|| async {
                (
                    axum::http::StatusCode::PERMANENT_REDIRECT,
                    [(header::LOCATION, "/demo/redirect/v")],
                )
            }),
        )
        .route(
            "/demo/redirect/v7",
            get(|| async {
                (
                    axum::http::StatusCode::TEMPORARY_REDIRECT,
                    [(header::LOCATION, "/demo/redirect/v")],
                )
            }),
        )
        .route(
            "/demo/redirect/v2",
            get(|| async { Redirect::temporary("/demo/redirect/v") }),
        )
        .route(
            "/demo/redirect/v3",
            get(|| async { Redirect::to("/demo/redirect/v") }),
        )
        .route(
            "/demo/redirect/post_repeat",
            get(demo_handler::show_repeat_html),
        )
        .route(
            "/demo/redirect/post_repeat",
            post(demo_handler::repeat_post),
        )
}

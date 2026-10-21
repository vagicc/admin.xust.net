use crate::handlers::column_handler;
use crate::models::column_model::GetQuery;
use axum::extract::{Form, Path, Query};
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/column/create", get(column_handler::create))
        .route("/column/create", post(column_handler::new))
        .route("/column/edit/{id}", get(column_handler::edit))
        .route("/column/edit/{id}", post(column_handler::do_edit))
        .route("/column/delete/{id}", get(column_handler::delete))
        .route(
            "/column/expurgate",
            post(
                |session: crate::session::Session,
                 Form(form): Form<std::collections::HashMap<String, String>>| async move {
                    let del = form.get("del_ids");
                    let mut ids: Vec<i32> = Vec::new();
                    if let Some(del) = del {
                        ids = del
                            .split(',')
                            .filter_map(|id| id.parse::<i32>().ok())
                            .collect();
                    }
                    column_handler::expurgate(ids, session).await
                },
            ),
        )
        .route(
            "/column/index",
            get(
                |Query(get): Query<GetQuery>, session: crate::session::Session| async move {
                    column_handler::list_page(1, get, session).await
                },
            ),
        )
        .route(
            "/column/index/{page}",
            get(
                |Path(page): Path<u32>,
                 Query(get): Query<GetQuery>,
                 session: crate::session::Session| async move {
                    column_handler::list_page(page, get, session).await
                },
            ),
        )
}

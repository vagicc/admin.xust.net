use crate::models::rights_model;
use crate::session::Session;
use crate::template::{to_html_single, view};
use axum::extract::{Form, Path};
use axum::response::{Html, IntoResponse, Redirect, Response};
use handlebars::to_json;
use serde_json::value::Map;

pub async fn edit(Path(id): Path<i32>, session: Session) -> Response {
    let mut data = Map::new();
    let edit = rights_model::find_right(id);
    if edit.is_none() {
        log::warn!("查无此数据:rights表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/rights/index"));
        data.insert("message".to_string(), to_json("查无此数据:rights表"));
        let html = to_html_single("hint.html", data);
        return Html(html).into_response();
    }

    data.insert("edit".to_string(), to_json(edit.unwrap()));
    let html = view("rights/rights_edit.html", data, session);
    Html(html).into_response()
}

pub async fn create_html(session: Session) -> Response {
    let data = Map::new();
    let html = view("rights/rights_create.html", data, session);
    Html(html).into_response()
}

pub async fn delete(Path(id): Path<i32>, _session: Session) -> Response {
    let _ = rights_model::delete(id);
    Redirect::to("/rights/index").into_response()
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RightPost {
    pub right_name: String,
    pub path_full: String,
    pub right_detail: String,
}
impl RightPost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.path_full.is_empty() {
            return Err("路径必填");
        }
        Ok(self.clone())
    }
}

pub async fn do_edit(
    Path(id): Path<i32>,
    _session: Session,
    Form(form): Form<RightPost>,
) -> Response {
    if let Ok(post) = form.validate() {
        let update = rights_model::UpdateRights {
            right_name: Some(post.right_name),
            path_full: post.path_full,
            right_detail: Some(post.right_detail),
        };
        let _ = rights_model::modify(id, &update);
    }
    Redirect::to("/rights/index").into_response()
}

pub async fn do_new(_session: Session, Form(form): Form<RightPost>) -> Response {
    if let Ok(post) = form.validate() {
        let new_data = rights_model::NewRights {
            right_name: Some(post.right_name.clone()),
            path_full: post.path_full.clone(),
            right_detail: Some(post.right_name.clone()),
        };
        let _ = new_data.insert();
    }
    Redirect::to("/rights/index").into_response()
}

pub async fn list(page: u32, session: Session) -> Response {
    let (count, list, pages) = rights_model::list(Some(page), crate::constants::PER_PAGE);
    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count));
    data.insert("list".to_string(), to_json(list));
    data.insert("pages".to_string(), to_json(pages));
    let html = view("rights/rights_list.html", data, session);
    Html(html).into_response()
}

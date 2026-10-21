use crate::models::admins_model;
use crate::session::Session;
use crate::template::{to_html_single, view};
use axum::extract::{Form, Path};
use axum::response::{Html, IntoResponse, Redirect, Response};
use handlebars::to_json;
use serde_json::value::Map;

pub async fn list(page: u32, session: Session) -> Response {
    log::debug!("后台用户列表-分页");
    let (count, list, pages) = admins_model::list(Some(page), crate::constants::PER_PAGE);
    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count));
    data.insert("list".to_string(), to_json(list));
    data.insert("pages".to_string(), to_json(pages));
    let html = view("admins/admins_list.html", data, session);
    Html(html).into_response()
}

pub async fn create_html(session: Session) -> Response {
    use crate::models::roles_model::get_all_role;
    let all_roles = get_all_role();
    let mut data = Map::new();
    data.insert("all_roles".to_string(), to_json(all_roles));
    let html = view("admins/admins_create.html", data, session);
    Html(html).into_response()
}

pub async fn do_new(_session: Session, Form(form): Form<admins_model::AdminPost>) -> Response {
    if let Ok(post) = form.validate() {
        let salt = admins_model::get_new_salt();
        let passwd = admins_model::encryption(&post.password, &salt);
        let new_data = admins_model::NewAdmin {
            username: post.username,
            password: passwd,
            salt,
            email: Some(post.email),
            mobile: Some(post.mobile),
            role: Some(post.role),
            status: Some(post.status),
            create_time: None,
            last_login: None,
        };
        let _ = new_data.insert();
    }
    Redirect::to("/admins/index").into_response()
}

pub async fn edit(Path(id): Path<i32>, session: Session) -> Response {
    let mut data = Map::new();
    let edit = admins_model::find_admin(id);
    if edit.is_none() {
        log::warn!("查无此数据:rights表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/admins/index"));
        data.insert("message".to_string(), to_json("查无此数据:Admins表"));
        let html = to_html_single("hint.html", data);
        return Html(html).into_response();
    }
    use crate::models::roles_model::get_all_role;
    let all_roles = get_all_role();
    data.insert("all_roles".to_string(), to_json(all_roles));
    data.insert("edit".to_string(), to_json(edit.unwrap()));
    let html = view("admins/admins_edit.html", data, session);
    Html(html).into_response()
}

pub async fn do_edit(
    Path(id): Path<i32>,
    _session: Session,
    Form(form): Form<admins_model::AdminPost>,
) -> Response {
    if let Ok(post) = form.validate() {
        let _ = admins_model::edit(id, &post);
    }
    Redirect::to("/admins/index").into_response()
}

pub async fn delete(Path(id): Path<i32>, _session: Session) -> Response {
    let _ = admins_model::delete(id);
    Redirect::to("/admins/index").into_response()
}

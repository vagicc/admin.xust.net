use crate::common::remove_menus_cache;
use crate::models::menus_model;
use crate::session::Session;
use crate::template::{to_html_single, view};
use axum::extract::{Form, Path};
use axum::response::{Html, IntoResponse, Redirect, Response};
use handlebars::to_json;
use serde_json::value::Map;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MenuPost {
    pub order_by: i16,
    pub path_full: String,
    pub name: String,
    pub level: i16,
    pub parent: i32,
    pub icon: String,
    pub department: i32,
    pub is_show: bool,
}
impl MenuPost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.name.is_empty() {
            return Err("菜单名必填");
        }
        Ok(self.clone())
    }
}

pub async fn list(parent_id: i32, session: Session) -> Response {
    let mut parent_data: Option<menus_model::Menu> = None;
    if parent_id > 0 {
        parent_data = menus_model::find_menu(parent_id);
    }
    let list = menus_model::get_parent(parent_id);
    let mut data = Map::new();
    data.insert("list".to_string(), to_json(list));
    data.insert("parent_data".to_string(), to_json(parent_data));
    data.insert("parent_id".to_string(), to_json(parent_id));
    Html(view("menus/menus_list.html", data, session)).into_response()
}

pub async fn create_html(Path(parent_id): Path<i32>, session: Session) -> Response {
    let department = menus_model::get_menu_level(1);
    let left_menus = menus_model::role_left_menus(1);
    let mut data = Map::new();
    data.insert("department".to_string(), to_json(department));
    data.insert("menus".to_string(), to_json(left_menus));
    data.insert("parent_id".to_string(), to_json(parent_id));
    let html = view("menus/menus_create.html", data, session);
    Html(html).into_response()
}

pub async fn do_new(
    Path(parent_id): Path<i32>,
    _session: Session,
    Form(form): Form<MenuPost>,
) -> Response {
    if let Ok(post) = form.validate() {
        let data = menus_model::NewMenu {
            order_by: post.order_by,
            path_full: Some(post.path_full),
            name: post.name,
            level: Some(post.level),
            parent: Some(post.parent),
            icon: Some(post.icon),
            department: Some(post.department),
            is_show: post.is_show,
        };
        let _ = data.insert();
        remove_menus_cache();
    }
    Redirect::to("/menus/index").into_response()
}

pub async fn edit(Path(id): Path<i32>, session: Session) -> Response {
    let mut data = Map::new();
    let edit = menus_model::find_menu(id);
    if edit.is_none() {
        log::warn!("查无此数据:menus表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/rights/index"));
        data.insert("message".to_string(), to_json("查无此数据:menus表"));
        let html = to_html_single("hint.html", data);
        return Html(html).into_response();
    }
    data.insert("edit".to_string(), to_json(edit.unwrap()));
    let left_menus = menus_model::role_left_menus(1);
    data.insert("menus".to_string(), to_json(left_menus));
    let department = menus_model::get_menu_level(1);
    data.insert("department".to_string(), to_json(department));
    let html = view("menus/menus_edit.html", data, session);
    Html(html).into_response()
}

pub async fn do_edit(
    Path(id): Path<i32>,
    _session: Session,
    Form(form): Form<MenuPost>,
) -> Response {
    if let Ok(post) = form.validate() {
        let update = menus_model::NewMenu {
            order_by: post.order_by,
            path_full: Some(post.path_full),
            name: post.name,
            level: Some(post.level),
            parent: Some(post.parent),
            icon: Some(post.icon),
            department: Some(post.department),
            is_show: post.is_show,
        };
        let _ = menus_model::modify(id, &update);
        remove_menus_cache();
    }
    Redirect::to("/menus/index").into_response()
}

pub async fn delete(Path(id): Path<i32>, _session: Session) -> Response {
    let _ = menus_model::delete(id);
    remove_menus_cache();
    Redirect::to("/menus/index").into_response()
}

pub async fn expurgate(ids: Vec<i32>, _session: Session) -> Response {
    for id in ids {
        let _ = menus_model::delete(id);
    }
    remove_menus_cache();
    Redirect::to("/menus/index").into_response()
}

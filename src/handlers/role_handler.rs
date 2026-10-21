use crate::models::roles_model;
use crate::session::Session;
use crate::template::{to_html_single, view};
use axum::extract::{Form, Path};
use axum::response::{Html, IntoResponse, Redirect, Response};
use handlebars::to_json;
use serde_json::value::Map;

pub async fn create_html(session: Session) -> Response {
    use crate::models::rights_model::get_all_right;
    let all_right = get_all_right();

    let mut data = Map::new();
    data.insert("all_right".to_string(), to_json(all_right));
    let html = view("roles/role_create.html", data, session);
    Html(html).into_response()
}

pub async fn list(page: u32, session: Session) -> Response {
    log::debug!("角色列表-分页");
    let (count, list, pages) = roles_model::list(Some(page), Some(crate::constants::PER_PAGE));
    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count));
    data.insert("list".to_string(), to_json(list));
    data.insert("pages".to_string(), to_json(pages));
    let html = view("roles/role_list.html", data, session);
    Html(html).into_response()
}

pub async fn edit(Path(id): Path<i32>, session: Session) -> Response {
    let mut data = Map::new();
    let edit = roles_model::find_role(id);
    if edit.is_none() {
        log::warn!("查无此数据:rights表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/role/index"));
        data.insert("message".to_string(), to_json("查无此数据:Roles表"));
        let html = to_html_single("hint.html", data);
        return Html(html).into_response();
    }
    let edit = edit.unwrap();
    use crate::models::rights_model::get_all_right;
    let all_right = get_all_right();
    let mut checkbox_data: Vec<CheckboxData> = Vec::new();
    if all_right.is_some() {
        checkbox_data = checkbox_is_checked(all_right.unwrap(), &edit.rights);
    }
    data.insert("all_right".to_string(), to_json(checkbox_data));
    data.insert("edit".to_string(), to_json(edit));
    let html = view("roles/role_edit.html", data, session);
    Html(html).into_response()
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct CheckboxData {
    right_id: i32,
    right_name: Option<String>,
    path_full: String,
    right_detail: Option<String>,
    is_checked: bool,
}

fn checkbox_is_checked(
    all: Vec<crate::models::rights_model::Rights>,
    checked: &Option<Vec<Option<i32>>>,
) -> Vec<CheckboxData> {
    let mut data: Vec<CheckboxData> = Vec::new();
    for rights in all {
        let mut is_checked = false;
        if checked.is_some() {
            let k = checked.clone().unwrap();
            is_checked = is_in_vec(&k.clone(), rights.right_id)
        }
        data.push(CheckboxData {
            right_id: rights.right_id,
            right_name: rights.right_name,
            path_full: rights.path_full,
            right_detail: rights.right_detail,
            is_checked,
        });
    }
    data
}

fn is_in_vec(arr: &Vec<Option<i32>>, search: i32) -> bool {
    for i in arr.iter() {
        if i.is_none() {
            continue;
        }
        if search == i.unwrap() {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RolePost {
    pub name: String,
    pub rights: String,
    pub default: String,
}
impl RolePost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.name.is_empty() {
            return Err("角色名必填");
        }
        if self.rights.is_empty() {
            return Err("权限不能为空");
        }
        Ok(self.clone())
    }
}

pub async fn do_edit(
    Path(id): Path<i32>,
    _session: Session,
    Form(form): Form<RolePost>,
) -> Response {
    if let Ok(post) = form.validate() {
        let rights: Vec<i32> = post
            .rights
            .split(',')
            .map(|x| x.parse::<i32>().expect("字符串转i32类型出错"))
            .collect();
        let new = roles_model::NewRole {
            name: post.name,
            rights: Some(rights),
            default: Some(post.default),
        };
        let _ = roles_model::modify(id, &new);
    }
    Redirect::to("/role/index").into_response()
}

pub async fn do_new(_session: Session, Form(form): Form<RolePost>) -> Response {
    if let Ok(post) = form.validate() {
        let rights: Vec<i32> = post
            .rights
            .split(',')
            .map(|x| x.parse::<i32>().expect("字符串转i32类型出错"))
            .collect();
        let data = roles_model::NewRole {
            name: post.name,
            rights: Some(rights),
            default: Some(post.default),
        };
        let _ = data.insert();
    }
    Redirect::to("/role/index").into_response()
}

pub async fn delete(Path(id): Path<i32>, _session: Session) -> Response {
    let _ = roles_model::delete(id);
    Redirect::to("/role/index").into_response()
}

pub async fn expurgate(ids: Vec<i32>, _session: Session) -> Response {
    for id in ids {
        let _ = roles_model::delete(id);
    }
    Redirect::to("/role/index").into_response()
}

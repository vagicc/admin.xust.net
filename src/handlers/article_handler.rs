use crate::models::article_category_m;
use crate::models::article_content_m;
use crate::models::article_model;
use crate::models::column_model;
use crate::session::Session;
use crate::template::to_html_single;
use crate::template::view;
use axum::extract::{Form, Path};
use axum::response::{Html, IntoResponse, Redirect, Response};
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;

pub async fn list_page(page: u32, get: article_model::GetQuery, session: Session) -> Response {
    log::debug!("文章列表GET: /book/list");

    let (count, list, pages) = article_model::list_page(
        Some(page),
        Some(crate::constants::PER_PAGE),
        Some(get.clone()),
    );

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count));
    data.insert("list".to_string(), to_json(list));
    data.insert("pages".to_string(), to_json(pages));
    data.insert("get".to_string(), to_json(get));

    let html = view("article/list.html", data, session);
    Html(html).into_response()
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArticlePost {
    pub title: String,
    pub username: String,
    pub nav_id: Option<i32>,
    pub category_id: Option<i32>,
    pub category: Option<String>,
    pub columns_id: i32,
    pub available: i16,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
}
impl ArticlePost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.title.is_empty() {
            return Err("文章标题不能为空");
        }
        Ok(self.clone())
    }
}

pub async fn create(session: Session) -> Response {
    log::error!("[调试信息]访问了“/article/create/”");

    let mut data = Map::new();
    let category = article_category_m::all_article_category();
    data.insert("category".to_string(), to_json(category));
    let column = column_model::all_Column_id_title();
    data.insert("column".to_string(), to_json(column));
    let html = view("article/create.html", data, session);
    Html(html).into_response()
}

pub async fn new(_session: Session, Form(form): Form<ArticlePost>) -> Response {
    log::debug!("接收到的post:{:#?}", form);
    let mut message = String::new();

    match form.validate() {
        Ok(post) => {
            let time = crate::common::time() as i64;
            let now = crate::common::now_naive_date_time();
            let data = article_model::NewArticle {
                title: post.title,
                cover: None,
                summary: post.summary,
                seo_title: post.seo_title,
                seo_keywords: post.seo_keywords,
                seo_description: post.seo_description,
                category_id: post.category_id,
                category: None,
                columns_id: post.columns_id,
                available: Some(post.available),
                nav_id: post.nav_id,
                visit: 0,
                collect: 0,
                share: 0,
                user_id: None,
                username: None,
                create: Some(time),
                last_time: Some(now),
            };
            let article_id = data.insert();
            if article_id == 0 {
                message = format!("插入文章出错：{}", article_id);
            } else if post.content.is_some() {
                let new = article_content_m::ArticleContent {
                    article_id,
                    content: post.content.unwrap(),
                    last_time: Some(now),
                };
                if new.insert() == 0 {
                    message = "插入文章内容出错".to_string();
                } else {
                    message = "新增文章成功！！".to_string();
                }
            }
        }
        Err(e) => {
            message = format!("POST表单认证不通过：{}", e);
        }
    }

    let mut data = Map::new();
    data.insert("jump_url".to_string(), to_json("/article/index"));
    data.insert("message".to_string(), to_json(message));
    let html = to_html_single("hint.html", data);
    Html(html).into_response()
}

pub async fn edit(Path(id): Path<i32>, session: Session) -> Response {
    let mut data = Map::new();
    let edit = article_model::get_article(id);
    if edit.is_none() {
        log::warn!("查无此数据:article表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/article/index"));
        data.insert("message".to_string(), to_json("查无此数据:article表"));
        let html = to_html_single("hint.html", data);
        return Html(html).into_response();
    }

    let content = article_content_m::get_article_content(id);
    let category = article_category_m::all_article_category();
    data.insert("category".to_string(), to_json(category));
    let column = column_model::all_Column_id_title();
    data.insert("column".to_string(), to_json(column));
    data.insert("edit".to_string(), to_json(edit.unwrap()));
    data.insert("article_content".to_string(), to_json(content));
    let html = view("article/edit.html", data, session);
    Html(html).into_response()
}

pub async fn do_edit(
    Path(id): Path<i32>,
    _session: Session,
    Form(form): Form<ArticlePost>,
) -> Response {
    let mut message = String::new();
    match form.validate() {
        Ok(post) => {
            let now = crate::common::now_naive_date_time();
            let new_article = article_model::NewArticle {
                title: post.title,
                cover: None,
                summary: post.summary,
                seo_title: post.seo_title,
                seo_keywords: post.seo_keywords,
                seo_description: post.seo_description,
                category_id: post.category_id,
                category: post.category,
                columns_id: post.columns_id,
                available: Some(post.available),
                nav_id: post.nav_id,
                visit: 0,
                collect: 0,
                share: 0,
                user_id: None,
                username: None,
                create: None,
                last_time: Some(now),
            };
            let updated = article_model::modify(id, &new_article);
            if updated.is_none() {
                message = "文章修改出错".to_string();
            } else {
                if let Some(content) = post.content {
                    let data = article_content_m::ArticleContent {
                        article_id: id,
                        content,
                        last_time: Some(now),
                    };
                    if article_content_m::modify(id, &data).is_none() {
                        message = "文章修改失败".to_string();
                    } else {
                        message = "文章修改成功".to_string();
                    }
                }
            }
        }
        Err(e) => {
            message = format!("文章修改POST表单认证不通过：{}", e);
        }
    }
    let mut data = Map::new();
    data.insert("jump_url".to_string(), to_json("/article/index"));
    data.insert("message".to_string(), to_json(message));
    let html = to_html_single("hint.html", data);
    Html(html).into_response()
}

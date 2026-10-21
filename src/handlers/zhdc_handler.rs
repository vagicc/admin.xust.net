use crate::models::reptile_zhdc_books_m;
use crate::models::reptile_zhdc_chapters_m;
use crate::session::Session;
use crate::template::to_html_single;
use crate::template::view;
use axum::extract::{Form, Path};
use axum::response::{Html, IntoResponse, Redirect, Response};
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;

pub async fn list_page(
    page: u32,
    get: reptile_zhdc_books_m::GetQuery,
    session: Session,
) -> Response {
    log::debug!("GET: /reptile/zhonghuadiancang");
    let (count, list, pages) = reptile_zhdc_books_m::list_page(
        Some(page),
        Some(crate::constants::PER_PAGE),
        Some(get.clone()),
    );
    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count));
    data.insert("list".to_string(), to_json(list));
    data.insert("pages".to_string(), to_json(pages));
    data.insert("get".to_string(), to_json(get));
    let html = view("zhdc/list.html", data, session);
    Html(html).into_response()
}

pub async fn new_html(session: Session) -> Response {
    log::debug!("[调试信息]访问了“/demo/redirect”");
    let data = Map::new();
    let html = view("zhdc/new.html", data, session);
    Html(html).into_response()
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewPost {
    pub url: String,
}
impl NewPost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.url.is_empty() {
            return Err("url不能为空");
        }
        Ok(self.clone())
    }
}

pub async fn new(_session: Session, Form(form): Form<NewPost>) -> Response {
    log::debug!("post:{:#?}", form);
    let mut message = String::new();
    match form.validate() {
        Ok(post) if !reptile_zhdc_books_m::whether_link_exists(post.url.clone()) => {
            let url = post.url.as_str();
            let result = crate::http::http_request(url).await;
            let response = result.unwrap();
            let html = response.html.as_str();
            let data = crate::parse::zhonghuadiancang_select(html).await;
            log::debug!("攫取到的：{:#?}", data);
            let zhdc_books_id = reptile_zhdc_books_m::NewReptileZhdcBooks {
                name: data.book_name.clone(),
                author: Some(data.book_author.clone()),
                publishing: None,
                front_cover: Some(data.front_cover.clone()),
                front_cover_download: Some(false),
                category: Some(data.category.clone()),
                description: Some(data.book_description.clone()),
                finish: Some(true),
                seo_title: Some(data.seo_title.clone()),
                seo_keywords: Some(data.seo_keywords.clone()),
                seo_description: Some(data.seo_description.clone()),
                reptile_url: url.to_string(),
                is_published: Some(false),
                create_time: None,
            }
            .insert();
            log::warn!("插入书目录ID：{}", zhdc_books_id);
            if zhdc_books_id == 0 {
                log::debug!("目录插入表出错！");
                message = "目录插入表出错！".to_string();
            }
            for chapter in data.book_chapters {
                let chapter_url = chapter.url;
                let result = crate::http::http_request(&chapter_url).await;
                let response = result.unwrap();
                let html = response.html.as_str();
                let chapter_data = crate::parse::zhdc_book_chapter_select(html).await;
                let insert_id = reptile_zhdc_chapters_m::NewReptileZhdcChapters {
                    zhdc_books_id,
                    book_name: Some(data.book_name.clone()),
                    title: chapter_data.title,
                    content: Some(chapter_data.content),
                    publish: Some(false),
                    seo_title: Some(chapter_data.seo_title),
                    seo_keywords: Some(chapter_data.seo_keywords),
                    seo_description: Some(chapter_data.seo_description),
                    reptile_url: chapter_url,
                    create_time: None,
                }
                .insert();
                log::warn!("插入书章节ID：{}", insert_id);
                if insert_id == 0 {
                    log::error!("章节插入表出错！");
                }
            }
            message = format!("书籍抓取成功，插入ID{}", zhdc_books_id);
            log::debug!("{}", message);
        }
        Ok(_) => {
            message = "可能存在过抓取的，就跑到这里处理了".to_string();
        }
        Err(_e) => {}
    }
    log::debug!("{}", message);
    let mut data = Map::new();
    data.insert(
        "jump_url".to_string(),
        to_json("/reptile/zhonghuadiancang/new"),
    );
    data.insert("message".to_string(), to_json(message));
    let html = to_html_single("hint.html", data);
    Html(html).into_response()
}

pub async fn book(Path(book_id): Path<i32>, session: Session) -> Response {
    let book = reptile_zhdc_books_m::find_book(book_id);
    let chapters = reptile_zhdc_chapters_m::get_book_chapters(book_id);
    let mut data = Map::new();
    data.insert("book".to_string(), to_json(book));
    data.insert("chapters".to_string(), to_json(chapters));
    let html = view("zhdc/book.html", data, session);
    Html(html).into_response()
}

pub async fn book_publish(Path(book_id): Path<i32>, _session: Session) -> Response {
    let message = if reptile_zhdc_books_m::publish_book(book_id, true, None) {
        format!("书籍ID：{} 发布成功!", book_id)
    } else {
        format!("书籍ID：{} 失败了", book_id)
    };
    log::debug!("{},整本发布end", message);
    let mut data = Map::new();
    data.insert("jump_url".to_string(), to_json("/reptile/zhonghuadiancang"));
    data.insert("message".to_string(), to_json(message));
    let html = to_html_single("hint.html", data);
    Html(html).into_response()
}

pub async fn book_chapter_publish(Path(chapter_id): Path<i32>, _session: Session) -> Response {
    let mut message = "未想清楚，未完成".to_string();
    let mut jump_url = "/reptile/zhonghuadiancang/chapter/publish/".to_string();
    log::debug!("一章发布start");
    if let Some(chapter) = reptile_zhdc_chapters_m::update_publish(chapter_id, true) {
        let zhdc_book =
            reptile_zhdc_books_m::find_book(chapter.zhdc_books_id).expect("爬虫表查无此书籍");
        if zhdc_book.is_published.is_none() || !zhdc_book.is_published.unwrap() {
            log::debug!("书未发布");
            reptile_zhdc_books_m::publish_book(chapter.zhdc_books_id, false, Some(chapter_id));
        }
        message = format!("章节ID：{} 发布成功!", chapter_id);
        jump_url = format!("/reptile/zhonghuadiancang/book/{}", chapter.zhdc_books_id);
    }
    log::debug!("一章发布end");
    let mut data = Map::new();
    data.insert("jump_url".to_string(), to_json(jump_url));
    data.insert("message".to_string(), to_json(message));
    let html = to_html_single("hint.html", data);
    Html(html).into_response()
}

pub async fn delete(Path(id): Path<i32>, _session: Session) -> Response {
    let mut message = String::new();
    let mut delete_row = reptile_zhdc_books_m::delete(id);
    if delete_row > 0 {
        delete_row += reptile_zhdc_chapters_m::delete_book(id);
        log::warn!("删除了{}章！", delete_row);
    }
    message = format!("ID{},删除了{}条数据", id, delete_row);
    let mut data = Map::new();
    data.insert("jump_url".to_string(), to_json("/reptile/zhonghuadiancang"));
    data.insert("message".to_string(), to_json(message));
    let html = to_html_single("hint.html", data);
    Html(html).into_response()
}

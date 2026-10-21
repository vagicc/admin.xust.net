use crate::session::Session;
use crate::template::view;
use axum::extract::Form;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use serde_json::value::Map;

pub async fn new_html(session: Session) -> Response {
    log::debug!("访问了：“/check/link”");

    let mut data = Map::new();
    let html = view("check/link.html", data, session);
    Html(html).into_response()
}

#[derive(Debug, Clone, serde::Deserialize)]
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

pub async fn check_all_url(_session: Session, Form(form): Form<NewPost>) -> Response {
    log::debug!("post:{:#?}", form);
    match form.validate() {
        Ok(post) => {
            let url = post.url;
            let links = get_all_url(url).await;
            if links.is_none() {
                println!("为何为空：{:#?}", links);
            }
            let mut links = links.unwrap();
            println!("上传的：{:#?}", links);
            let mut post: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();
            println!("上传的表单：{:#?}", post);
        }
        Err(_e) => {}
    }
    let html = "检查URL所有的死链".to_string();
    Html(html).into_response()
}

async fn next_level_url(links: &mut std::collections::HashSet<url::Url>, depth: usize) {
    if depth > 58 {
        return;
    }
    for link in links.clone().iter() {
        let kk = link.host_str();
        if kk != Some("www.59fayi.com") {
            continue;
        }
        let url = link.as_str().to_string();
        if !url.contains("/m/") {
            continue;
        }
        println!("<li><a href=\"{}\">{0}</a></li>", url);
        if let Some(temp) = get_all_url(url).await {
            let mut difference: std::collections::HashSet<url::Url> =
                temp.difference(&links).cloned().collect();
            Box::pin(next_level_url(&mut difference, depth + 1)).await;
            links.extend(difference);
        }
    }
}

// Unused old traversal helper left for reference.
// async fn _old_next_level_url(
//     links: &std::collections::HashSet<url::Url>,
//     depth: usize,
// ) -> std::collections::HashSet<url::Url> {
//     let mut temp_links: std::collections::HashSet<url::Url> = std::collections::HashSet::new();
//     if depth > 58 {
//         return temp_links;
//     }
//     for link in links {
//         let kk = link.host_str();
//         if kk != Some("www.59fayi.com") {
//             continue;
//         }
//         let url = link.as_str().to_string();
//         if !url.contains("/m/") {
//             continue;
//         }
//         if !temp_links.contains(&link) {
//             println!("<li><a href=\"{}\">{0}</a></li>", url);
//             temp_links.insert(link.clone());
//             if let Some(temp) = get_all_url(url).await {
//                 temp_links.extend(temp.clone());
//                 let result = Box::pin(_old_next_level_url(&difference, depth + 1)).await;
//                 temp_links.extend(result);
//             }
//         }
//     }
//     temp_links
// }

async fn get_all_url(url: String) -> Option<std::collections::HashSet<url::Url>> {
    let mut links: std::collections::HashSet<url::Url> = std::collections::HashSet::new();
    let res = reqwest::get(&url).await;
    match res {
        Ok(res) => {
            let res = res.text().await.expect("msg");
            let document = select::document::Document::from(res.as_str());
            for node in document.find(select::predicate::Name("a")) {
                if let Some(href) = node.attr("href") {
                    if let Ok(parsed) = url::Url::parse(href) {
                        links.insert(parsed);
                    }
                }
            }
            Some(links)
        }
        Err(_) => None,
    }
}

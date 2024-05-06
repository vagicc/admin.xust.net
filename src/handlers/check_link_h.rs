use crate::session::Session;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};

/* 响应： /check/link */
pub async fn new_html(session: Session) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("访问了：“/check/link”");

    let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);
    let html = view("check/link.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回状态码
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct NewPost {
    pub url: String, //要抓的目录URL
}
impl NewPost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.url.is_empty() {
            return Err("url不能为空");
        }
        Ok(self.clone())
    }
}

pub async fn check_all_url(
    form: NewPost,
    _session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("post:{:#?}", form);
    match form.validate() {
        Ok(post) => {
            let url = post.url;
            let links = get_all_url(url).await;
            let links = links.unwrap();
            let next = next_level_url(&links, 1).await;

            println!("第一层URL");
            for link in links {
                let url = link.as_str().to_string();
                println!("{}", url);
            }
            println!("第=====二=============层URL");
            for link in next {
                let url = link.as_str().to_string();
                println!("{}", url);
            }
        }
        Err(e) => {}
    }
    let mut html = "检查URL所有的死链".to_string();
    Ok(warp::reply::html(html)) //直接返回html
}

//这里是异步递归
async fn next_level_url(
    links: &std::collections::HashSet<url::Url>,
    depth: usize,
) -> std::collections::HashSet<url::Url> {
    let mut temp_links: std::collections::HashSet<url::Url> = std::collections::HashSet::new();
    if depth > 2 {
        return temp_links;
    }
    for link in links {
        /// let url = Url::parse("ftp://rms@example.com")?;
        /// assert_eq!(url.host_str(), Some("example.com"));
        let kk = link.host_str();
        if kk!=Some("www.59fayi.com"){
            // println!("别的域名:{:#?}，不做抓取", kk);
            continue;
        }
        // println!("域名：{:#?}",kk);
        let url = link.as_str().to_string();
        // println!("{}", url);
        println!("<li><a href=\"{}\">{0}</a></li>",url);
        let temp_option = get_all_url(url).await;
        if let Some(temp) = temp_option {
            temp_links.extend(temp.clone()); //当前层

            // 异步递归
            let future = Box::pin(next_level_url(&temp, depth + 1));
            let result = future.await;
            temp_links.extend(result);
        }
    }
    temp_links
}

async fn get_all_url(url: String) -> Option<std::collections::HashSet<url::Url>> {
    let mut links: std::collections::HashSet<url::Url> = std::collections::HashSet::new();
    let res = reqwest::get(&url).await;
    match res {
        Ok(res) => {
            let res = res.text().await.expect("msg");
            let document = select::document::Document::from(res.as_str());
            let temp_url = url::Url::parse(&url).unwrap();
            let base_url = get_base_url(&temp_url, &document);
            // println!("取得的基URL：{:#?}", base_url);
            let base_parser = url::Url::options().base_url(Some(&base_url));
            //取得当前页的URL
            let links: std::collections::HashSet<url::Url> = document
                .find(select::predicate::Name("a"))
                .filter_map(|n| n.attr("href"))
                .filter_map(|link| base_parser.parse(link).ok())
                .collect();
            // println!("当前页所有的URL：{:#?}", links);
            Some(links)
        }
        Err(e) => None,
    }
}

fn get_base_url(url: &url::Url, doc: &select::document::Document) -> url::Url {
    let base_tag_href = doc
        .find(select::predicate::Name("base"))
        .filter_map(|n| n.attr("href"))
        .nth(0);

    let base_url = base_tag_href
        .map_or_else(
            || url::Url::parse(&url[..url::Position::BeforePath]),
            url::Url::parse,
        )
        .expect("取得base_url出错");
    base_url
}

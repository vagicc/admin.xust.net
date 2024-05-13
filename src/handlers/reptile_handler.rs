use crate::models::reptile_zhdc_books_m;
use crate::models::reptile_zhdc_chapters_m;
use crate::session::Session;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};

pub async fn test_html_select(_: Session) -> std::result::Result<impl Reply, Rejection> {
    Ok(warp::reply::html("这里用来测试HTML")) //直接返回html
}

/* 响应： new_html*/
pub async fn new_html(session: Session) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("[调试信息]访问了“/demo/redirect”");

    let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);
    let html = view("reptile/new.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回状态码
}

//https://www.zhonghuadiancang.com/xuanxuewushu/18616/
//https://www.zhonghuadiancang.com/xuanxuewushu/18783/
//https://www.zhonghuadiancang.com/xueshuzaji/18404/
//https://www.zhonghuadiancang.com/xueshuzaji/18289/

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

//处理抓取“中华典藏”
pub async fn zhonghuadiancang(
    form: NewPost,
    session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("post:{:#?}", form);
    match form.validate() {
        Ok(post) if !reptile_zhdc_books_m::whether_link_exists(post.url.clone()) => {
            let url = post.url.as_str();
            //先判断是否存在
            // let k=reptile_zhdc_books_m::whether_link_exists(url.to_string());

            let result = crate::http::http_request(url).await;
            let response = result.unwrap();
            // println!("response: {:?}", response);
            let html = response.html.as_str();
            // println!("抓取到的html=========={}", html);
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
            }

            //开始循环去抓取详情页  ………………
            for chapter in data.book_chapters {
                let chapter_url = chapter.url;
                let result = crate::http::http_request(&chapter_url).await;
                let response = result.unwrap();
                // println!("response: {:?}", response);
                let html = response.html.as_str();
                // println!("抓取到的html=========={}", html);
                let chapter_data = crate::parse::zhdc_book_chapter_select(html).await;
                //插入
                let insert_id = reptile_zhdc_chapters_m::NewReptileZhdcChapters {
                    zhdc_books_id: zhdc_books_id,
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
                log::debug!("插入书章节ID：{}", insert_id);

                if insert_id == 0 {
                    log::error!("章节插入表出错！");
                }
            }
            println!("完成插入");
        }
        Ok(_) => {
            println!("可能存在过抓取的，就跑到这里处理了");
        }
        Err(e) => {}
    }
    let mut html = "抓取书目录页".to_string();
    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn test_zhonghuadiancang_detail(
    _: Session,
) -> std::result::Result<impl Reply, Rejection> {
    let url = "https://www.zhonghuadiancang.com/xuanxuewushu/18783/344485.html".to_string();
    zhonghuadiancang_book_chapter(url).await;
    Ok(warp::reply::html("这里用来测试HTML")) //直接返回html
}

//抓取书详细章节内容
//测试url:https://www.zhonghuadiancang.com/xuanxuewushu/18783/344485.html
async fn zhonghuadiancang_book_chapter(url: String) {
    let result = crate::http::http_request(&url).await;
    let response = result.unwrap();
    // println!("response: {:?}", response);
    let html = response.html.as_str();

    // println!("抓取到的html=========={}", html);
    println!("到这===");
    let data = crate::parse::zhdc_book_chapter_select(html).await;
}

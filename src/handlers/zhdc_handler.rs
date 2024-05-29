use crate::models::reptile_zhdc_books_m;
use crate::models::reptile_zhdc_chapters_m;
use crate::session::Session;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};

//GET: /reptile/zhonghuadiancang
pub async fn list_page(
    page: u32,
    get: GetQuery,
    session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("GET: /reptile/zhonghuadiancang");

    // let per: u32 = 18; //每页总数
    let (count, list, pages) = reptile_zhdc_books_m::list_page(
        Some(page),
        Some(crate::constants::PER_PAGE),
        Some(get.clone()),
    );

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));
    data.insert("get".to_string(), to_json(get));

    // let html = to_html_single("reptile_new.html", data);
    let html = view("zhdc/list.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

// GET查询条件
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetQuery {
    pub book_name: Option<String>,  //书名
    pub is_published: Option<bool>, //是否已推送
}

/* 响应： new_html*/
pub async fn new_html(session: Session) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("[调试信息]访问了“/demo/redirect”");

    let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);
    let html = view("zhdc/new.html", data, session);

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

//https://www.zhonghuadiancang.com/xuanxuewushu/18616/
//https://www.zhonghuadiancang.com/xuanxuewushu/18783/
//https://www.zhonghuadiancang.com/xueshuzaji/18404/
//https://www.zhonghuadiancang.com/xueshuzaji/18289/
//处理抓取“中华典藏”
pub async fn new(form: NewPost, session: Session) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("post:{:#?}", form);
    let mut message = String::new();

    match form.validate() {
        //只处理目录URL不存在过抓取的
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
                message = "目录插入表出错！".to_string();
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
        Err(e) => {}
    }
    log::debug!("{}", message);

    // let mut html = "抓取书目录页".to_string();
    let mut data = Map::new();
    data.insert(
        "jump_url".to_string(),
        to_json("/reptile/zhonghuadiancang/new"),
    );
    data.insert("message".to_string(), to_json(message));

    let html = to_html_single("hint.html", data);
    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn book(book_id: i32, session: Session) -> std::result::Result<impl Reply, Rejection> {
    // 取得书籍信息，再取章节信息
    let book = reptile_zhdc_books_m::find_book(book_id);
    if book.is_none() {
        log::warn!("Book {} not found,无此书", book_id);
        // Err(())
        //
    }

    let chapters = reptile_zhdc_chapters_m::get_book_chapters(book_id);

    let mut data = Map::new();
    data.insert("book".to_string(), to_json(book)); //
    data.insert("chapters".to_string(), to_json(chapters)); //

    // let html = to_html_single("reptile_new.html", data);
    let html = view("zhdc/book.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

//整本书籍发布
//GET: reptile/zhonghuadiancang/publish/{{id}}
pub async fn book_publish(
    book_id: i32,
    session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::error!("整本发布start");
    let k = reptile_zhdc_books_m::publish_book(book_id, true);
    log::error!("整本发布end");

    // let html = to_html_single("reptile_new.html", data);
    // let html = view("zhdc/book.html", data, session);
    let html = "整本发布";
    Ok(warp::reply::html(html)) //直接返回html
}

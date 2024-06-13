use crate::db::get_connection;
use crate::schema::reptile_zhdc_books;
use crate::schema::reptile_zhdc_books::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct ReptileZhdcBooks {
    pub id: i32,
    pub name: String,
    pub author: Option<String>,
    pub publishing: Option<String>,
    pub front_cover: Option<String>,
    pub front_cover_download: Option<bool>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub finish: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub reptile_url: String,
    pub is_published: Option<bool>,
    pub create_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = reptile_zhdc_books)]
pub struct NewReptileZhdcBooks {
    pub name: String,
    pub author: Option<String>,
    pub publishing: Option<String>,
    pub front_cover: Option<String>,
    pub front_cover_download: Option<bool>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub finish: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub reptile_url: String,
    pub is_published: Option<bool>,
    pub create_time: Option<NaiveDateTime>,
}
impl NewReptileZhdcBooks {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        let query = diesel::insert_into(reptile_zhdc_books)
            .values(self)
            .returning(id);
        log::debug!(
            "reptile_zhdc_books表插入数据SQL：{:?}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );
        let result = query.get_result::<i32>(&mut connection);
        match result {
            Ok(insert_id) => {
                log::debug!("插入成功，ID为：{}", insert_id);
                insert_id
            }
            Err(err) => {
                //value too long for type character varying(255) 字段太短，插入内容太长
                log::error!("插入数据失败了：{}", err);
                0
            }
        }
    }
}

pub fn whether_link_exists(url: String) -> bool {
    let query = reptile_zhdc_books.filter(reptile_url.eq(url));
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("whether_link_exists查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<ReptileZhdcBooks>(&mut connection);
    match result {
        Ok(row) => {
            log::debug!("reptile_zhdc_books存在数据");
            true
        }
        Err(err) => {
            log::debug!("reptile_zhdc_books查无数据：{}", err);
            false
        }
    }
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list_page(
    page: Option<u32>,
    per: Option<u32>,
    whe: Option<crate::handlers::zhdc_handler::GetQuery>,
) -> (i64, Vec<ReptileZhdcBooks>, String) {
    let mut limit: i64 = 50; //每页取几条数据
    let mut offset: i64 = 0; //从第0条开始

    if !per.is_none() {
        limit = per.unwrap() as i64;
    }

    if !page.is_none() && page.unwrap() > 1 {
        offset = ((page.unwrap() as i64) - 1) * limit;
    }

    let mut query = reptile_zhdc_books.into_boxed();
    let mut query_count = reptile_zhdc_books.into_boxed();
    //可变的查询条件以上面结合下面的写法
    if let Some(params) = whe {
        if let Some(book_name) = params.book_name.filter(|n| !n.is_empty()) {
            let name_like = format!("%{}%", book_name);
            query = query.filter(name.like(name_like.clone()));
            query_count = query_count.filter(name.like(name_like));
        }
        if let Some(published) = params.is_published {
            query = query.filter(is_published.eq(published));
            query_count = query_count.filter(is_published.eq(published));
        }
    }

    let query_count = query_count.count();
    log::error!(
        "reptile_zhdc_books分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("reptile_zhdc_books分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<ReptileZhdcBooks> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = query
        .order_by(id.desc())
        .limit(limit) //取10条数据
        .offset(offset); //从第0条开始;
    log::error!(
        "reptile_zhdc_books分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query
        .get_results::<ReptileZhdcBooks>(&mut conn)
        .unwrap_or(data_null);

    // let page = page.unwrap_or(1);
    pages = crate::pager::default_full(
        "reptile/zhonghuadiancang",
        count,
        page.unwrap_or(1),
        limit as u32,
    );
    (count, list, pages)
}

pub fn find_book(book_id: i32) -> Option<ReptileZhdcBooks> {
    let query = reptile_zhdc_books.find(book_id);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("find_book查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<ReptileZhdcBooks>(&mut connection);
    match result {
        Ok(row) => Some(row),
        Err(err) => {
            log::debug!("find_book查无数据：{}", err);
            None
        }
    }
}

// 发布 publish
// publish_all:true全部发布，false只发布一章
// chapter_id：只发布的章节ID
//(publish_all为false,并传入chapter_id时，只发布当前章节)
pub fn publish_book(book_id: i32, publish_all: bool, chapter_id: Option<i32>) -> bool {
    match find_book(book_id) {
        Some(book) => {
            //开启事务，发布书，all为true时再连同所有章节发布。最后修改本表为已发布
            use crate::models::books_model;
            let createid = 8;
            let new_book = books_model::NewBooks {
                name: book.name.clone(),
                author: book.author.clone(),
                publisher: book.publishing.clone(),
                front_cover: book.front_cover.clone(),
                price: None,
                category_id: None,
                category: book.category.clone(),
                description: book.description.clone(),
                finish: Some(publish_all), //是否已完结
                collect: None,
                seo_title: book.seo_title.clone(),
                seo_keywords: book.seo_keywords.clone(),
                seo_description: book.seo_description.clone(),
                create_id: Some(createid),
                create_time: None,
            };
            let new_book_id = new_book.insert();
            if new_book_id == 0 {
                log::error!("插入到书籍表出错！！");
                return false;
            }

            //================================start===================

            //发布所有的章节
            use crate::models::book_chapters_content_m;
            use crate::models::book_chapters_m;
            use crate::models::reptile_zhdc_chapters_m;
            let chapters = reptile_zhdc_chapters_m::get_book_chapters(book_id);
            if chapters.is_none() {
                return true;
            }
            let mut previous: i32 = 0;
            for chapter in chapters.unwrap() {
                let mut p = publish_all;
                if !publish_all && chapter_id.is_some() {
                    if chapter_id.unwrap() == chapter.id {
                        p = true;
                    }
                }
                let new_chapter = book_chapters_m::NewBookChapters {
                    book_id: new_book_id,
                    book_name: chapter.book_name,
                    author: book.author.clone(),
                    title: chapter.title,
                    // content: chapter.content,  //这里要写到新的表“”
                    visit: 0,
                    previous: Some(previous), //上一章（ID）
                    next: None,               //下一章（ID）
                    publish: Some(p),         //是否发布
                    seo_title: chapter.seo_title,
                    seo_keywords: chapter.seo_keywords,
                    seo_description: chapter.seo_description,
                    create_id: Some(createid),
                    create: None, //创建时间( Unix 时间戳)
                    last_time: None,
                };
                let insert_id = new_chapter.insert();
                if let Some(content) = chapter.content {
                    if !content.is_empty() {
                        let new_content = book_chapters_content_m::BookChaptersContent {
                            chapter_id: insert_id,
                            content: content,
                            last_time: None,
                        };
                        let _ = new_content.insert();
                    }
                }
                if previous > 0 {
                    //更新下一章。
                    book_chapters_m::update_next(previous, insert_id);
                }
                previous = insert_id;
            }
            //更新所有章节为已发布
            reptile_zhdc_chapters_m::update_book_publish(book_id, true);
            //================================end===================

            //更新为已发布
            update_published(book_id, true);

            true
        }
        None => false,
    }
}

pub fn update_published(pky: i32, published: bool) {
    let query = diesel::update(reptile_zhdc_books.find(pky)).set(is_published.eq(published));
    log::error!(
        "reptile_zhdc_books表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    let _ = query.execute(&mut conn);
}

pub fn delete(pky: i32) -> usize {
    let query = diesel::delete(reptile_zhdc_books.find(pky));
    log::debug!(
        "reptile_zhdc_books表删除SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );
    let mut conn = get_connection();
    let deleted_rows = query.execute(&mut conn);
    // crate::common::type_v(deleted_rows);
    //变量值：Ok(1)  =>类型： core::result::Result<usize, diesel::result::Error>  删除成功1条数据
    //变量值：Ok(0)  =>类型： core::result::Result<usize, diesel::result::Error>  删除成功0条数据

    match deleted_rows {
        Ok(row) => row,
        Err(e) => {
            log::error!("reptile_zhdc_books表删除数据失败：{}", e);
            0
        }
    }
}

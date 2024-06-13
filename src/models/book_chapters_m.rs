use crate::db::get_connection;
use crate::schema::book_chapters;
use crate::schema::book_chapters::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct BookChapters {
    pub id: i32,
    pub book_id: i32,
    pub book_name: Option<String>,
    pub author: Option<String>,
    pub title: String,
    // pub content: Option<String>,  //独立到表：book_chapters_content
    pub visit: i64,
    pub previous: Option<i32>,
    pub next: Option<i32>,
    pub publish: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub create_id: Option<i32>,
    pub create: Option<i64>,
    pub last_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = book_chapters)]
pub struct NewBookChapters {
    pub book_id: i32,
    pub book_name: Option<String>,
    pub author: Option<String>,
    pub title: String,
    // pub content: Option<String>,
    pub visit: i64,
    pub previous: Option<i32>,
    pub next: Option<i32>,
    pub publish: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub create_id: Option<i32>,
    pub create: Option<i64>,
    pub last_time: Option<NaiveDateTime>,
}
impl NewBookChapters {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        let query = diesel::insert_into(book_chapters)
            .values(self)
            .returning(id);
        log::debug!(
            "book_chapters表插入数据SQL：{:?}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );
        let result = query.get_result::<i32>(&mut connection);
        match result {
            Ok(insert_id) => {
                log::debug!("book_chapters插入成功，ID为：{}", insert_id);
                insert_id
            }
            Err(err) => {
                //value too long for type character varying(255) 字段太短，插入内容太长
                log::error!("book_chapters插入数据失败了：{}", err);
                0
            }
        }
    }
}

//更新下章ID
pub fn update_next(pky: i32, next_id: i32) {
    let query = diesel::update(book_chapters.find(pky)).set(next.eq(next_id));
    log::debug!(
        "book_chapters表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    let k = query.execute(&mut conn);
}

//查找本书籍所有章节
pub fn get_book_all_chapters(bookid: i32) -> Option<Vec<BookChapters>> {
    let query = book_chapters.filter(book_id.eq(bookid));
    log::debug!(
        "get_book_all_chapters数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut connection = get_connection();

    match query.get_results::<BookChapters>(&mut connection) {
        Ok(list) => Some(list),
        Err(err) => {
            log::debug!("get_book_all_chapters查无数据：{}", err);
            None
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
    whe: Option<crate::handlers::book_chapters_h::GetQuery>,
) -> (i64, Vec<BookChapters>, String) {
    let mut limit: i64 = 50; //每页取几条数据
    let mut offset: i64 = 0; //从第0条开始

    if !per.is_none() {
        limit = per.unwrap() as i64;
    }

    if !page.is_none() && page.unwrap() > 1 {
        offset = ((page.unwrap() as i64) - 1) * limit;
    }

    let mut query = book_chapters.into_boxed();
    let mut query_count = book_chapters.into_boxed();

    if let Some(params) = whe {
        //章节标题
        if let Some(c_title) = params.title.filter(|s| !s.is_empty()) {
            let title_like = format!("%{}%", c_title);
            query = query.filter(title.like(title_like.clone()));
            query_count = query_count.filter(title.like(title_like));
        }
        //书籍ID
        if let Some(b_id) = params.book_id.filter(|c| *c > 0) {
            query = query.filter(book_id.eq(b_id));
            query_count = query_count.filter(book_id.eq(b_id));
        }
        //书名
        if let Some(bookname) = params.book_name.filter(|bn| !bn.is_empty()) {
            let name_like = format!("%{}%", bookname);
            query = query.filter(book_name.like(name_like.clone()));
            query_count = query_count.filter(book_name.like(name_like));
        }
        //作者
        if let Some(book_author) = params.book_author.filter(|s| !s.is_empty()) {
            let author_like = format!("%{}%", book_author);
            query = query.filter(author.like(author_like.clone()));
            query_count = query_count.filter(author.like(author_like));
        }
        //是否已发布
        if let Some(finish_whe) = params.publish.filter(|f| *f > 0) {
            let finish_whe = if finish_whe == 1 { true } else { false };
            query = query.filter(publish.eq(finish_whe));
            query_count = query_count.filter(publish.eq(finish_whe));
        }
    }

    let query_count = query_count.count();
    log::warn!(
        "book_chapters分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("book_chapters分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<BookChapters> = Vec::new();
    if count <= 0 {
        // return (count, data_null, pages);
    }

    let query = query
        .order_by(id.desc())
        .limit(limit) //取10条数据
        .offset(offset); //从第0条开始;

    log::warn!(
        "book_chapters分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query
        .get_results::<BookChapters>(&mut conn)
        .unwrap_or(data_null);

    pages =
        crate::pager::default_full("book/chapters/list", count, page.unwrap_or(1), limit as u32);

    (count, list, pages)
}

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
    pub book_id: Option<i32>,
    pub book_name: Option<String>,
    pub author: Option<String>,
    pub title: String,
    pub content: Option<String>,
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
    pub book_id: Option<i32>,
    pub book_name: Option<String>,
    pub author: Option<String>,
    pub title: String,
    pub content: Option<String>,
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

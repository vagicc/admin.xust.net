use crate::db::get_connection;
use crate::schema::book_chapters_content;
use crate::schema::book_chapters_content::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，Queryable：查询,AsChangeset修改) */
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = book_chapters_content)]
pub struct BookChaptersContent {
    pub chapter_id: i32,
    pub content: String,
    pub last_time: Option<NaiveDateTime>,
}
impl BookChaptersContent {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        let query = diesel::insert_into(book_chapters_content)
            .values(self)
            .returning(chapter_id);
        log::debug!(
            "book_chapters_content表插入数据SQL：{:?}",
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

pub fn find_chapters_content(chapterid: i32) -> Option<BookChaptersContent> {
    let query = book_chapters_content.find(chapterid);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("find_chapters_content查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<BookChaptersContent>(&mut connection);
    match result {
        Ok(row) => Some(row),
        Err(err) => {
            log::debug!("find_book查无数据：{}", err);
            None
        }
    }
}

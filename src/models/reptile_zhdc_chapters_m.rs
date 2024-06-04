use crate::db::get_connection;
use crate::schema::reptile_zhdc_chapters;
use crate::schema::reptile_zhdc_chapters::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct ReptileZhdcChapters {
    pub id: i32,
    pub zhdc_books_id: i32,
    pub book_name: Option<String>,
    pub title: String,
    pub content: Option<String>,
    pub publish: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub reptile_url: String,
    pub create_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = reptile_zhdc_chapters)]
pub struct NewReptileZhdcChapters {
    pub zhdc_books_id: i32,
    pub book_name: Option<String>,
    pub title: String,
    pub content: Option<String>,
    pub publish: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub reptile_url: String,
    pub create_time: Option<NaiveDateTime>,
}
impl NewReptileZhdcChapters {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        diesel::insert_into(reptile_zhdc_chapters)
            .values(self)
            .returning(id)
            .get_result::<i32>(&mut connection)
            .unwrap_or(0)
    }
}

// 取得此书所有章节
pub fn get_book_chapters(book_id: i32) -> Option<Vec<ReptileZhdcChapters>> {
    let query = reptile_zhdc_chapters
        .filter(zhdc_books_id.eq(book_id))
        .order_by(id.asc());
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_book_chapters查询SQL：{:?}", sql);

    let mut connection = get_connection();

    match query.get_results::<ReptileZhdcChapters>(&mut connection) {
        Ok(list) => Some(list),
        Err(err) => {
            log::debug!("get_book_chapters查无数据：{}", err);
            None
        }
    }
}

//更新所有章节为已发布
pub fn update_book_publish(book_id: i32, published: bool) {
    let query = diesel::update(reptile_zhdc_chapters.filter(zhdc_books_id.eq(book_id)))
        .set(publish.eq(published));
    log::error!(
        "reptile_zhdc_chapters表更新书籍所有章节发布状态SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    let _ = query.execute(&mut conn);
}

//更新指定章节为已发布
pub fn update_publish(pky: i32, published: bool) -> Option<ReptileZhdcChapters> {
    let query = diesel::update(reptile_zhdc_chapters.find(pky)).set(publish.eq(published));
    log::error!(
        "reptile_zhdc_chapters表更新指定章节发布状态SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );
    let mut conn = get_connection();
    match query.get_result::<ReptileZhdcChapters>(&mut conn) {
        Ok(result) => Some(result),
        Err(err) => {
            log::error!("reptile_zhdc_chapters表修改数据失败：{}", err);
            None
        }
    }
}

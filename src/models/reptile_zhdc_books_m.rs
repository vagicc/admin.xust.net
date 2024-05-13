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

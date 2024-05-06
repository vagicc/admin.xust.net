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

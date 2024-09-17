use crate::db::get_connection;
use crate::schema::column;
use crate::schema::column::dsl::*;
use chrono::NaiveDateTime;
use diesel::data_types::Cents;
use diesel::prelude::*;
// use serde::{Deserialize, Serialize};
// use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::ser::{Serialize, SerializeStruct, Serializer};

/* 表查询插入结构体(Insertable：插入，Queryable：查询) */
// #[derive(Debug, Clone, Queryable, Serialize)] //Cents類型在Serialize會出錯，要手動添加
#[derive(Debug, Clone, Queryable)]
pub struct Column {
    pub id: i32,
    pub title: String,
    pub subhead: String,
    pub surface_plot: Option<String>,
    pub author: Option<String>,
    pub excerpt: Option<String>,
    // price -> Nullable<Money>,
    // pub price: Option<WrappedCents>,
    pub price: Option<Cents>,
    pub visit: i64,
    pub collect: i64,
    pub amount: Option<i32>,
    pub complete: i32,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub create_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}
//手動添加Serialize特征，Cents無“Serialize”特征
impl Serialize for Column {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Person", 16).unwrap();
        s.serialize_field("id", &self.id).unwrap();
        s.serialize_field("title", &self.title).unwrap();
        s.serialize_field("subhead", &self.subhead).unwrap();
        s.serialize_field("surface_plot", &self.surface_plot)
            .unwrap();
        s.serialize_field("author", &self.author).unwrap();
        s.serialize_field("excerpt", &self.excerpt).unwrap();

        // s.serialize_field("price", &self.price).unwrap();
        s.serialize_field("price", &((self.price.unwrap().0 as f64) / 100.))
            .unwrap();

        s.serialize_field("visit", &self.visit).unwrap();
        s.serialize_field("collect", &self.collect).unwrap();
        s.serialize_field("amount", &self.amount).unwrap();
        s.serialize_field("complete", &self.complete).unwrap();
        s.serialize_field("seo_title", &self.seo_title).unwrap();
        s.serialize_field("seo_keywords", &self.seo_keywords)
            .unwrap();
        s.serialize_field("seo_description", &self.seo_description)
            .unwrap();
        s.serialize_field("create_id", &self.create_id).unwrap();
        s.serialize_field("create_time", &self.create_time).unwrap();

        s.end()
    }
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = column)]
pub struct NewColumn {
    pub title: String,
    pub subhead: String,
    pub surface_plot: Option<String>,
    pub author: Option<String>,
    pub excerpt: Option<String>,
    // price -> Nullable<Money>,
    // pub price: Option<WrappedCents>,
    pub price: Option<Cents>,
    pub visit: i64,
    pub collect: i64,
    pub amount: Option<i32>,
    pub complete: i32,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub create_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}
impl NewColumn {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        let query = diesel::insert_into(column).values(self).returning(id);
        log::debug!(
            "column表插入数据SQL：{:?}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );
        let result = query.get_result::<i32>(&mut connection);
        match result {
            Ok(insert_id) => {
                log::debug!("column插入成功，ID为：{}", insert_id);
                insert_id
            }
            Err(err) => {
                //value too long for type character varying(255) 字段太短，插入内容太长
                log::error!("column插入数据失败了：{}", err);
                0
            }
        }
    }
}

//後臺列表GET查詢條件
#[derive(Debug, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct GetQuery {
    pub title: Option<String>,   //标题
    pub subhead: Option<String>, //副标题
    pub author: Option<String>,  //作者
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list_page(
    page: Option<u32>,
    per: Option<u32>,
    whe: Option<GetQuery>,
) -> (i64, Vec<Column>, String) {
    let mut limit: i64 = 50; //每页取几条数据
    let mut offset: i64 = 0; //从第0条开始

    if !per.is_none() {
        limit = per.unwrap() as i64;
    }

    if !page.is_none() && page.unwrap() > 1 {
        offset = ((page.unwrap() as i64) - 1) * limit;
    }

    let mut query = column.into_boxed();
    let mut query_count = column.into_boxed();
    //可变的查询条件以上面结合下面的写法
    if let Some(params) = whe {
        if let Some(subhead_like) = params.subhead.filter(|t| !t.is_empty()) {
            let subhead_like = format!("%{}%", subhead_like);
            query = query.filter(subhead.like(subhead_like.clone()));
            query_count = query_count.filter(subhead.like(subhead_like));
        }

        if let Some(title_like) = params.title.filter(|t| !t.is_empty()) {
            let title_like = format!("%{}%", title_like);
            query = query.filter(title.like(title_like.clone()));
            query_count = query_count.filter(title.like(title_like));
        }

        if let Some(author_query) = params.author.filter(|t| !t.is_empty()) {
            let author_query = format!("%{}%", author_query);
            query = query.filter(author.eq(author_query.clone()));
            query_count = query_count.filter(author.eq(author_query));
        }

        // //是否显示：默认1显示，0不显示
        // if let Some(a) = params.show.filter(|a| *a < 2) {
        //     query = query.filter(show.eq(a));
        //     query_count = query_count.filter(show.eq(a));
        // }
    }

    let query_count = query_count.count();
    log::debug!(
        "column分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("column分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<Column> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = query
        .order_by(id.desc())
        .limit(limit) //取10条数据
        .offset(offset); //从第0条开始;
    log::debug!(
        "column分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query.get_results::<Column>(&mut conn).unwrap_or(data_null);

    pages = crate::pager::default_full("column/index", count, page.unwrap_or(1), limit as u32);

    (count, list, pages)
}

pub fn modify(pk: i32, data: &NewColumn) -> Option<Column> {
    let query = diesel::update(column.find(pk)).set(data);
    log::debug!(
        "column表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    match query.get_result::<Column>(&mut conn) {
        Ok(result) => Some(result),
        Err(err) => {
            log::error!("column表修改数据失败：{}", err);
            None
        }
    }
}

//删除一条记录
pub fn delete(pky: i32) -> usize {
    let query = diesel::delete(column.find(pky));
    log::debug!(
        "column表删除SQL：{:?}",
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
            log::error!("column表删除数据失败：{}", e);
            0
        }
    }
}

/// 通过ID查找详情
pub fn get_column(pky: i32) -> Option<Column> {
    let query = column.find(pky);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_column查询SQL：{:?}", sql);

    let mut connection = get_connection();
    let result = query.first::<Column>(&mut connection);

    match result {
        Ok(data) => Some(data),
        Err(e) => {
            log::debug!("get_column查无数据：{}", e);
            return None;
        }
    }
}

#[derive(Debug, Clone, Queryable, serde::Serialize)]
pub struct ColumnIdName {
    pub id: i32,
    pub title: String,
}

pub fn all_Column_id_title() -> Option<Vec<ColumnIdName>> {
    let query = column.select((id, title)).order_by(id.desc());
    log::warn!(
        "all_Column_id_title表查詢SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );
    let mut conn = get_connection();
    let result = query.get_results::<ColumnIdName>(&mut conn);
    match result {
        Ok(data) => Some(data),
        Err(e) => {
            log::error!("article_category表查找所有分類失敗：{}", e);
            None
        }
    }
}

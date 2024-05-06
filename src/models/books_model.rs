use crate::db::get_connection;
use crate::schema::books;
use crate::schema::books::dsl::*;
use chrono::NaiveDateTime;
use diesel::data_types::Cents;
use diesel::prelude::*;
// use serde::{Deserialize};
use serde::ser::{Serialize, SerializeStruct, Serializer};

/* 表查询插入结构体(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Queryable)]
pub struct Books {
    pub id: i32,
    pub name: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub front_cover: Option<String>,
    pub price: Option<Cents>, // 要单独增加Serialize对应Cents
    pub category_id: Option<i32>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub finish: Option<bool>,
    pub collect: Option<i64>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub create_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}
// 手动国添加上Serialize特征: Cents与BigDecimal无特征,所以手动添加
impl Serialize for Books {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let num_elements = std::mem::size_of::<Books>() / std::mem::size_of::<i32>(); //取得结构体的元素个数
        let mut s = serializer.serialize_struct("Person", num_elements).unwrap();
        s.serialize_field("id", &self.id).unwrap();
        s.serialize_field("name", &self.name).unwrap();
        s.serialize_field("author", &self.author).unwrap();
        s.serialize_field("publisher", &self.publisher).unwrap();
        s.serialize_field("front_cover", &self.front_cover).unwrap();

        let price_temp = (self.price.unwrap().0 as f64) / 100.;
        s.serialize_field("price", &price_temp).unwrap();

        s.serialize_field("category_id", &self.category_id).unwrap();
        s.serialize_field("category", &self.category).unwrap();
        s.serialize_field("description", &self.description).unwrap();
        s.serialize_field("finish", &self.finish).unwrap();
        s.serialize_field("collect", &self.collect).unwrap();
        s.serialize_field("seo_title", &self.seo_title).unwrap();
        s.serialize_field("seo_keywords", &self.seo_keywords).unwrap();
        s.serialize_field("seo_description", &self.seo_description).unwrap();
        s.serialize_field("create_id", &self.create_id).unwrap();
        s.serialize_field("create_time", &self.create_time).unwrap();

        s.end()
    }
}

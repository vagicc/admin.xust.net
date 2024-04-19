// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Int4,
        #[max_length = 16]
        username -> Varchar,
        #[max_length = 40]
        password -> Varchar,
        #[max_length = 10]
        salt -> Bpchar,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        #[max_length = 11]
        mobile -> Nullable<Bpchar>,
        role -> Nullable<Int4>,
        status -> Nullable<Int8>,
        create_time -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
    }
}

diesel::table! {
    book_category (id) {
        id -> Int4,
        #[max_length = 20]
        category -> Varchar,
        #[max_length = 255]
        seo_title -> Nullable<Varchar>,
        #[max_length = 255]
        seo_keywords -> Nullable<Varchar>,
        #[max_length = 255]
        seo_description -> Nullable<Varchar>,
        show -> Nullable<Bool>,
        order_by -> Nullable<Int2>,
        modify_id -> Nullable<Int4>,
        modify_time -> Nullable<Timestamp>,
        create_id -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    book_chapters (id) {
        id -> Int4,
        bood_id -> Nullable<Int4>,
        bood_name -> Nullable<Int4>,
        #[max_length = 180]
        author -> Nullable<Varchar>,
        #[max_length = 255]
        title -> Varchar,
        content -> Nullable<Text>,
        visit -> Int8,
        previous -> Nullable<Int4>,
        next -> Nullable<Int4>,
        #[max_length = 255]
        seo_title -> Nullable<Varchar>,
        #[max_length = 255]
        seo_keywords -> Nullable<Varchar>,
        #[max_length = 255]
        seo_description -> Nullable<Varchar>,
        create_id -> Nullable<Int4>,
        create -> Nullable<Int8>,
        last_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 180]
        author -> Nullable<Varchar>,
        #[max_length = 255]
        publishing -> Nullable<Varchar>,
        #[max_length = 255]
        front_cover -> Nullable<Varchar>,
        price -> Nullable<Money>,
        category_id -> Nullable<Int4>,
        #[max_length = 20]
        category -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        finish -> Nullable<Bool>,
        collect -> Nullable<Int8>,
        #[max_length = 255]
        seo_title -> Nullable<Varchar>,
        #[max_length = 255]
        seo_keywords -> Nullable<Varchar>,
        #[max_length = 255]
        seo_description -> Nullable<Varchar>,
        create_id -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ci_sessions (id) {
        #[max_length = 128]
        id -> Varchar,
        ip_address -> Inet,
        timestamp -> Timestamptz,
        data -> Bytea,
    }
}

diesel::table! {
    menus (id) {
        id -> Int4,
        order_by -> Int2,
        #[max_length = 255]
        path_full -> Nullable<Varchar>,
        #[max_length = 20]
        name -> Varchar,
        level -> Nullable<Int2>,
        parent -> Nullable<Int4>,
        #[max_length = 50]
        icon -> Nullable<Varchar>,
        department -> Nullable<Int4>,
        is_show -> Bool,
    }
}

diesel::table! {
    record (record_time) {
        id -> Int4,
        table_id -> Int4,
        #[max_length = 180]
        table_name -> Varchar,
        user_id -> Int4,
        #[max_length = 18]
        username -> Varchar,
        #[max_length = 180]
        action -> Varchar,
        ip -> Inet,
        record_time -> Timestamp,
    }
}

diesel::table! {
    rights (right_id) {
        right_id -> Int4,
        #[max_length = 30]
        right_name -> Nullable<Varchar>,
        #[max_length = 255]
        path_full -> Varchar,
        #[max_length = 30]
        right_detail -> Nullable<Varchar>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 20]
        name -> Varchar,
        rights -> Nullable<Array<Nullable<Int4>>>,
        #[max_length = 50]
        default -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    book_category,
    book_chapters,
    books,
    ci_sessions,
    menus,
    record,
    rights,
    roles,
);

// @generated automatically by Diesel CLI.

diesel::table! {
    blogs (id) {
        id -> Int4,
        user_id -> Int4,
        create_date -> Timestamptz,
        updated_date -> Timestamptz,
        title -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        blog_id -> Int4,
        create_date -> Timestamptz,
        updated_date -> Timestamptz,
        published_date -> Nullable<Timestamptz>,
        title -> Varchar,
        description -> Varchar,
        content -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    blogs,
    posts,
    users,
);

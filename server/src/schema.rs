diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        blog_id -> Int4,
        user_id -> Int4,
        create_date -> Timestamptz,
        last_mod_date -> Timestamptz,
        publish_date -> Timestamptz,
        title -> Varchar,
        description -> Text,
        content -> Text,
    }
}

diesel::table! {
    blogs (id) {
        id -> Int4,
        create_date -> Timestamptz,
        last_mod_date -> Timestamptz,
        publish_date -> Timestamptz,
        title -> Varchar,
        description -> Text,
    }
}

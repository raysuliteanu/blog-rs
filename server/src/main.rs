use chrono::{DateTime, Utc};
use diesel::prelude::*;
use schema::{blogs, posts, users};

mod schema;

joinable!(posts -> users (user_id));
allow_tables_to_appear_in_same_query!(posts, users);

joinable!(posts -> blogs (blog_id));
allow_tables_to_appear_in_same_query!(posts, blogs);

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    id: i32,
    name: String,
}

#[derive(Debug, Associations, Identifiable, Queryable, Insertable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Blog))]
#[diesel(belongs_to(User))]
struct Post {
    id: i32,
    blog_id: i32,
    user_id: i32,
    create_date: DateTime<Utc>,
    last_mod_date: DateTime<Utc>,
    publish_date: DateTime<Utc>,
    title: String,
    description: String,
    content: String,
}

#[derive(Debug, Identifiable, Queryable, Insertable)]
#[diesel(table_name = blogs)]
struct Blog {
    id: i32,
    create_date: DateTime<Utc>,
    last_mod_date: DateTime<Utc>,
    publish_date: DateTime<Utc>,
    title: String,
    description: String,
}

fn main() {
    println!("Hello, world!");
}

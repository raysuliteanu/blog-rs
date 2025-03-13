use crate::schema::{blogs, posts, users};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error as DieselError;

joinable!(blogs -> users (user_id));
joinable!(posts -> users (user_id));
joinable!(posts -> blogs (blog_id));

#[derive(Insertable, Identifiable, Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    id: i32,
    username: String,
}

#[derive(Debug, Associations, Identifiable, Queryable, Insertable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Blog))]
#[diesel(belongs_to(User))]
pub struct Post {
    id: i32,
    blog_id: i32,
    user_id: i32,
    create_date: DateTime<Utc>,
    updated_date: DateTime<Utc>,
    published_date: Option<DateTime<Utc>>,
    title: String,
    description: String,
    content: String,
}

#[derive(Debug, Identifiable, Queryable, Insertable, Selectable)]
#[diesel(table_name = blogs)]
pub struct Blog {
    id: i32,
    user_id: i32,
    create_date: DateTime<Utc>,
    updated_date: DateTime<Utc>,
    title: String,
    description: String,
}

/// get user by id; returns the user or NotFound error
pub fn get_user(user_id: i32, conn: &mut PgConnection) -> Result<User, DieselError> {
    use crate::schema::users::dsl::*;
    users.find(user_id).first(conn)
}

pub fn create_dummy_user() -> User {
    User {
        id: 1,
        username: "Alice".to_string(),
    }
}

pub fn create_dummy_blog() -> Blog {
    Blog {
        id: 1,
        user_id: 1,
        create_date: Utc::now(),
        updated_date: Utc::now(),
        title: "Alice's Blog".to_string(),
        description: "This is my blog".to_string(),
    }
}

pub fn create_dummy_posts() -> Vec<Post> {
    vec![
        Post {
            id: 1,
            blog_id: 1,
            user_id: 1,
            create_date: Utc::now(),
            updated_date: Utc::now(),
            published_date: None,
            title: "Hello, world!".to_string(),
            description: "This is a test post".to_string(),
            content: "This is the content of the post".to_string(),
        },
        Post {
            id: 2,
            blog_id: 1,
            user_id: 1,
            create_date: Utc::now(),
            updated_date: Utc::now(),
            published_date: None,
            title: "Hello, world!".to_string(),
            description: "This is a test post".to_string(),
            content: "This is the content of the post".to_string(),
        },
    ]
}

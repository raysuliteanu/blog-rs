use chrono::{DateTime, Utc};
use diesel::prelude::*;
use schema::{blogs, posts, users};

mod db;
mod schema;

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
struct Post {
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
struct Blog {
    id: i32,
    user_id: i32,
    create_date: DateTime<Utc>,
    updated_date: DateTime<Utc>,
    title: String,
    description: String,
}

fn main() {
    use schema::blogs::dsl::*;
    use schema::posts::dsl::*;
    use schema::users::dsl::*;

    let mut conn: PgConnection = db::establish_connection();

    let user = User {
        id: 1,
        username: "alice".to_string(),
    };

    diesel::insert_into(users)
        .values(&user)
        .execute(&mut conn)
        .expect("Error saving new user");

    diesel::insert_into(blogs)
        .values(&Blog {
            id: 1,
            user_id: 1,
            create_date: Utc::now(),
            updated_date: Utc::now(),
            title: "Alice's Blog".to_string(),
            description: "This is my blog".to_string(),
        })
        .execute(&mut conn)
        .expect("Error saving new blog");

    let vec = vec![
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
    ];
    diesel::insert_into(posts)
        .values(&vec)
        .execute(&mut conn)
        .expect("Error saving new post");

    let results = posts
        .select(Post::as_select())
        .load::<Post>(&mut conn)
        .unwrap();
    for post in results {
        println!("{:?}", post);
    }
}

use crate::schema::{blogs, posts, users};
use chrono::{DateTime, Utc};
use diesel::prelude::*;

pub type DieselResult<T> = Result<T, diesel::result::Error>;

#[derive(Insertable, Identifiable, Queryable, Selectable, PartialEq, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
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

#[derive(Debug, Associations, Identifiable, Queryable, Insertable, Selectable)]
#[diesel(table_name = blogs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct Blog {
    id: i32,
    user_id: i32,
    create_date: DateTime<Utc>,
    updated_date: DateTime<Utc>,
    title: String,
    description: String,
}

pub fn create_user(name: &str, conn: &mut PgConnection) -> DieselResult<User> {
    use crate::schema::users::username;
    diesel::insert_into(users::table)
        .values(username.eq(name))
        .returning(User::as_returning())
        .get_result(conn)
}
/// get user by id; returns the user or NotFound error
pub fn get_user(id: i32, conn: &mut PgConnection) -> DieselResult<User> {
    users::dsl::users.find(id).first(conn)
}

pub fn get_blogs(user: User, conn: &mut PgConnection) -> DieselResult<Vec<Blog>> {
    Blog::belonging_to(&user)
        .select(Blog::as_select())
        .load(conn)
}

pub fn get_pages(blog: &Blog, conn: &mut PgConnection) -> DieselResult<Vec<Post>> {
    Post::belonging_to(blog)
        .select(Post::as_select())
        .load(conn)
}

pub fn create_blog(
    user: &User,
    title: &str,
    desc: &str,
    conn: &mut PgConnection,
) -> DieselResult<Blog> {
    let create_date = Utc::now();
    let update_date = create_date;
    diesel::insert_into(blogs::table)
        .values((
            blogs::user_id.eq(user.id),
            blogs::create_date.eq(create_date),
            blogs::updated_date.eq(update_date),
            blogs::title.eq(title),
            blogs::description.eq(desc),
        ))
        .returning(Blog::as_returning())
        .get_result(conn)
}

pub fn create_post(
    blog: &Blog,
    title: &str,
    desc: &str,
    content: &str,
    conn: &mut PgConnection,
) -> DieselResult<Post> {
    let create_date = Utc::now();
    let update_date = create_date;
    diesel::insert_into(posts::table)
        .values((
            posts::user_id.eq(blog.user_id),
            posts::blog_id.eq(blog.id),
            posts::create_date.eq(create_date),
            posts::updated_date.eq(update_date),
            posts::title.eq(title),
            posts::description.eq(desc),
            posts::content.eq(content),
        ))
        .returning(Post::as_returning())
        .get_result(conn)
}

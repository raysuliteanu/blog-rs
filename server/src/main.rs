use diesel::prelude::*;
use model::Post;

mod db;
mod model;
mod schema;

fn main() {
    use schema::blogs::dsl::*;
    use schema::posts::dsl::*;
    use schema::users::dsl::*;

    let mut conn: PgConnection = db::establish_connection();

    let user = model::get_dummy_user();

    diesel::insert_into(users)
        .values(&user)
        .execute(&mut conn)
        .expect("Error saving new user");

    let blog = model::get_dummy_blog();

    diesel::insert_into(blogs)
        .values(&blog)
        .execute(&mut conn)
        .expect("Error saving new blog");

    let vec = model::get_dummy_posts();
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

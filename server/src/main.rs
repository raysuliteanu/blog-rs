use std::error::Error;

use diesel::prelude::*;
use log::{SetLoggerError, info};
use log4rs::{
    Config, Handle,
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
};
use model::Post;

mod db;
mod model;
mod schema;

fn main() -> Result<(), Box<dyn Error>> {
    init_logging()?;

    info!("starting");

    use schema::blogs::dsl::*;
    use schema::posts::dsl::*;
    use schema::users::dsl::*;

    let mut conn: PgConnection = db::establish_connection();

    let user = model::create_dummy_user();

    diesel::insert_into(users)
        .values(&user)
        .execute(&mut conn)
        .expect("Error saving new user");

    let blog = model::create_dummy_blog();

    diesel::insert_into(blogs)
        .values(&blog)
        .execute(&mut conn)
        .expect("Error saving new blog");

    let vec = model::create_dummy_posts();
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

    let user = model::get_user(1, &mut conn)?;
    info!("{user:?}");

    Ok(())
}

fn init_logging() -> Result<Handle, SetLoggerError> {
    const APPENDER_NAME: &str = "console";
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();

    let log_config = Config::builder()
        .appender(Appender::builder().build(APPENDER_NAME, Box::new(stdout)))
        .build(
            Root::builder()
                .appender(APPENDER_NAME)
                .build(log::LevelFilter::Debug),
        )
        .unwrap();

    log4rs::init_config(log_config)
}

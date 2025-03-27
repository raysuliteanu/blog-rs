use std::error::Error;

use crate::schema::users;
use diesel::prelude::*;
use log::{SetLoggerError, info};
use log4rs::{
    Config, Handle,
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
};

mod db;
mod model;
mod schema;

fn main() -> Result<(), Box<dyn Error>> {
    init_logging()?;

    info!("starting");

    let mut conn: PgConnection = db::establish_connection();

    let orig_users_cnt = model::User::get_users(&mut conn)?.len();
    let alice = model::User::create_user("Alice", &mut conn)?;
    let bob = model::User::create_user("Bob", &mut conn)?;
    let carol = model::User::create_user("Carol", &mut conn)?;

    let users = model::User::get_users(&mut conn)?;
    assert_eq!(users.len(), orig_users_cnt + 3);

    assert_eq!(
        "Bob",
        model::User::find_user_by_name(&bob.username, &mut conn)?.username
    );

    assert_eq!(
        "Carol",
        model::User::find_user_by_name(&carol.username, &mut conn)?.username
    );

    let orig_blogs_cnt = model::get_all_blogs(&mut conn)?.len();
    let alice_blog = model::create_blog(&alice, "Alice's blog", "blog by Alice", &mut conn)?;
    let _bob_blog = model::create_blog(&alice, "Bob's blog", "blog by Bob", &mut conn)?;
    let _carol_blog = model::create_blog(&alice, "Carol's blog", "blog by Carol", &mut conn)?;

    let blogs = model::get_all_blogs(&mut conn)?;
    assert_eq!(blogs.len(), orig_blogs_cnt + 3);

    let _p1 = model::create_post(
        &alice_blog,
        "the first post",
        "whatever",
        "some content",
        &mut conn,
    )?;

    let _p2 = model::create_post(
        &alice_blog,
        "the second post",
        "yada yada",
        "yada yada content",
        &mut conn,
    )?;

    let results = model::get_pages(&alice_blog, &mut conn)?;

    for post in results {
        println!("{:?}", post);
    }

    for user in users {
        println!("{user:?}");
    }

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

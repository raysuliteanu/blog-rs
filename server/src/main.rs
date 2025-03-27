use std::error::Error;

use diesel::prelude::*;
use log::{info, SetLoggerError};
use log4rs::{
    append::console::ConsoleAppender, config::{Appender, Root},
    encode::json::JsonEncoder,
    Config,
    Handle,
};

mod db;
mod model;
mod schema;

fn main() -> Result<(), Box<dyn Error>> {
    init_logging()?;

    info!("starting");

    let mut conn: PgConnection = db::establish_connection();

    let user = model::create_user("Alice", &mut conn)?;

    let blog = model::create_blog(&user, "Alice's blog", "blog by Alice", &mut conn)?;

    let _p1 = model::create_post(
        &blog,
        "the first post",
        "whatever",
        "some content",
        &mut conn,
    )?;

    let _p2 = model::create_post(
        &blog,
        "the second post",
        "yada yada",
        "yada yada content",
        &mut conn,
    )?;

    let results = model::get_pages(&blog, &mut conn)?;

    for post in results {
        println!("{:?}", post);
    }

    let user = model::get_user(user.id, &mut conn)?;
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

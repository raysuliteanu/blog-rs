use std::error::Error;

use log::{SetLoggerError, info};
use log4rs::{
    Config, Handle,
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
};

fn main() -> Result<(), Box<dyn Error>> {
    init_logging()?;

    info!("starting");

    info!("shutting down");

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

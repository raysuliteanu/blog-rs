use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = db_url();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// TODO: use config crate
fn db_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

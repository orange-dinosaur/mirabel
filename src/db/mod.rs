use crate::error::{Error, Result};
use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect_to_db() -> Result<DatabaseConnection> {
    println!("Connecting to the database...");

    let db_url = match env::var_os("DATABASE_URL") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$DATABASE_URL is not set"),
    };

    // To configure the connection the ConnectOptions interface can be used:
    // let mut opt = ConnectOptions::new("protocol://username:password@host/database");
    //    opt.max_connections(100)
    //    .min_connections(5)
    //    .connect_timeout(Duration::from_secs(8))
    //    .acquire_timeout(Duration::from_secs(8))
    //    .idle_timeout(Duration::from_secs(8))
    //    .max_lifetime(Duration::from_secs(8))
    //    .sqlx_logging(true)
    //    .sqlx_logging_level(log::LevelFilter::Info)
    //    .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema
    let db = Database::connect(db_url).await.map_err(Error::DbError)?;

    Ok(db)
}

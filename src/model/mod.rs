use crate::{db::connect_to_db, error::Result};
use sea_orm::DatabaseConnection;

pub mod books;

#[derive(Clone)]
pub struct ModelManager {
    db: DatabaseConnection,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self> {
        let db = connect_to_db().await?;
        println!("Connected to the database");

        Ok(ModelManager { db })
    }

    /// Returns a reference to the database pool
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}

use sqlx::{SqlitePool, migrate::MigrateDatabase, sqlite::SqlitePoolOptions};

use crate::{error::DatabaseError, repos::item::ItemRepo};

pub mod models {
    pub mod item;
}

pub mod repos {
    pub mod item;
}

pub mod error;

#[derive(Clone)]
pub struct Database {
    db: SqlitePool,
}

impl Database {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn create_connect_migrate(db_url: &str) -> Result<Self, DatabaseError> {
        // create database if not exists
        sqlx::Sqlite::create_database(db_url).await?;

        // connect to database
        let db = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        // run migrations
        sqlx::migrate!("../migrations").run(&db).await?;

        Ok(Self { db })
    }

    pub fn users<'a>(&'a self) -> ItemRepo<'a> {
        ItemRepo::new(&self.db)
    }
}

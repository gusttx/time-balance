pub mod models {
    mod activity;
    mod entry;

    pub use activity::Activity;
    pub use entry::Entry;
}

use crate::utils::DatabaseError;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    SqlitePool,
};
use std::marker::PhantomData;
use tauri::{AppHandle, Manager};

pub struct Initialized;
pub struct Uninitialized;

pub struct Database<State = Uninitialized> {
    pub pool: SqlitePool,
    state: PhantomData<State>,
}

impl Database<Uninitialized> {
    pub async fn new(app_handle: &AppHandle) -> Result<Self, DatabaseError> {
        let local_data = app_handle.path().app_local_data_dir()?;
        if !local_data.exists() {
            std::fs::create_dir_all(&local_data)?;
        }

        let database_path = local_data.join("database.db");
        log::debug!("Connecting to database...");

        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::new()
                .filename(&database_path)
                .journal_mode(SqliteJournalMode::Wal)
                .create_if_missing(true),
        )
        .await?;

        log::debug!("Connected to database at: {database_path:?}");

        Ok(Self {
            pool,
            state: PhantomData,
        })
    }

    pub async fn init(self) -> Result<Database<Initialized>, DatabaseError> {
        log::debug!("Executing migrations...");

        sqlx::migrate!("./migrations").run(&self.pool).await?;

        log::debug!("Migrations executed successfully.");

        Ok(Database {
            pool: self.pool,
            state: PhantomData,
        })
    }
}

pub type Db = Database<Initialized>;

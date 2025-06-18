use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Result};

use crate::database::Db;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Activity {
    pub id: i64,
    pub name: String,
    pub points_per_second: i8,
    pub deleted: bool,
}

impl Activity {
    fn new(id: i64, name: String, points_per_second: i8) -> Self {
        Self {
            id,
            name,
            points_per_second,
            deleted: false,
        }
    }

    pub async fn get(db: &Db, id: i64) -> Result<Option<Self>> {
        sqlx::query_as("SELECT * FROM activities WHERE id = ?")
            .bind(id)
            .fetch_optional(&db.pool)
            .await
    }

    pub async fn get_all(db: &Db) -> Result<Vec<Self>> {
        sqlx::query_as("SELECT * FROM activities WHERE deleted = 0 ORDER BY id DESC")
            .fetch_all(&db.pool)
            .await
    }

    pub async fn create(db: &Db, name: String, points_per_second: i8) -> Result<Self> {
        sqlx::query("INSERT INTO activities (name, points_per_second) VALUES (?, ?)")
            .bind(&name)
            .bind(points_per_second)
            .execute(&db.pool)
            .await
            .map(|r| Self::new(r.last_insert_rowid(), name, points_per_second))
    }

    pub async fn delete(db: &Db, id: i64) -> Result<()> {
        sqlx::query("UPDATE activities SET deleted = 1 WHERE id = ?")
            .bind(id)
            .execute(&db.pool)
            .await
            .map(|_| ())
    }

    pub async fn edit(db: &Db, id: i64, name: String, points_per_second: i8) -> Result<Self> {
        sqlx::query("UPDATE activities SET name = ?, points_per_second = ? WHERE id = ?")
            .bind(&name)
            .bind(points_per_second)
            .bind(id)
            .execute(&db.pool)
            .await
            .map(|_| Self::new(id, name, points_per_second))
    }
}

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Result};

use crate::database::{models::Activity, Db};

#[derive(Deserialize, Serialize, FromRow)]
pub struct Entry {
    pub id: i64,
    pub duration: i64,
    pub points_per_second: i8,
    pub activity_id: i64,
    pub activity_name: String,
}

impl Entry {
    fn new(
        id: i64,
        duration: i64,
        points_per_second: i8,
        activity_id: i64,
        activity_name: String,
    ) -> Self {
        Self {
            id,
            duration,
            points_per_second,
            activity_id,
            activity_name,
        }
    }

    pub async fn get_page(db: &Db, page: i64) -> Result<(Vec<Self>, i64)> {
        let entries: Vec<Entry> = sqlx::query_as(
            r#"
                SELECT
                    e.id AS id,
                    e.duration AS duration,
                    e.points_per_second AS points_per_second,
                    e.activity_id AS activity_id,
                    a.name AS activity_name
                FROM entries e
                JOIN activities a ON e.activity_id = a.id
                ORDER BY e.id DESC
                LIMIT 10 OFFSET ?
            "#,
        )
        .bind(page * 10)
        .fetch_all(&db.pool)
        .await?;

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM entries")
            .fetch_one(&db.pool)
            .await?;

        Ok((entries, total.0))
    }

    pub async fn create(db: &Db, duration: i64, activity: Activity) -> Result<Self> {
        sqlx::query(
            "INSERT INTO entries (duration, points_per_second, activity_id) VALUES (?, ?, ?)",
        )
        .bind(duration)
        .bind(activity.points_per_second)
        .bind(activity.id)
        .execute(&db.pool)
        .await
        .map(|result| {
            Self::new(
                result.last_insert_rowid(),
                duration,
                activity.points_per_second,
                activity.id,
                activity.name,
            )
        })
    }

    pub async fn delete(db: &Db, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM entries WHERE id = ?")
            .bind(id)
            .execute(&db.pool)
            .await
            .map(|_| ())
    }
}

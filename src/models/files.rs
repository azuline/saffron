use chrono::naive::NaiveDateTime;
use serde::Serialize;
use sqlx::{sqlite::SqliteRow, FromRow, Row, SqlitePool};

#[derive(FromRow, Serialize, Debug)]
pub struct File {
    pub id: i64,
    pub filename: String,
    pub uploader_id: i64,
    pub uploaded_on: NaiveDateTime,
}

impl File {
    pub async fn from_id(db_pool: &SqlitePool, id: i64) -> Option<Self> {
        sqlx::query(
            r#"
                SELECT
                    id,
                    filename,
                    uploader_id,
                    uploaded_on
                FROM files
                WHERE id = $1
            "#,
        )
        .bind(id)
        .map(|row: SqliteRow| File {
            id: row.get(0),
            filename: row.get(1),
            uploader_id: row.get(2),
            uploaded_on: row.get(3),
        })
        .fetch_one(db_pool)
        .await
        .ok()
    }

    pub async fn from_filename(db_pool: &SqlitePool, filename: &str) -> Option<Self> {
        sqlx::query(
            r#"
                SELECT
                    id,
                    filename,
                    uploader_id,
                    uploaded_on
                FROM files
                WHERE filename = $1
            "#,
        )
        .bind(filename)
        .map(|row: SqliteRow| File {
            id: row.get(0),
            filename: row.get(1),
            uploader_id: row.get(2),
            uploaded_on: row.get(3),
        })
        .fetch_one(db_pool)
        .await
        .ok()
    }

    pub async fn all_of_user(
        db_pool: &SqlitePool,
        user_id: i64,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query(
            r#"
                SELECT
                    id,
                    filename,
                    uploader_id,
                    uploaded_on
                FROM files
                WHERE uploader_id = $1
                ORDER BY id DESC
                LIMIT $2
                OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(per_page)
        .bind((page - 1) * per_page)
        .map(|row: SqliteRow| File {
            id: row.get(0),
            filename: row.get(1),
            uploader_id: row.get(2),
            uploaded_on: row.get(3),
        })
        .fetch_all(db_pool)
        .await
    }

    pub async fn count(db_pool: &SqlitePool, user_id: i64) -> Result<i64, sqlx::Error> {
        sqlx::query(
            r#"
                SELECT COUNT(1)
                FROM files
                WHERE uploader_id = $1
            "#,
        )
        .bind(user_id)
        .map(|row: SqliteRow| row.get(0))
        .fetch_one(db_pool)
        .await
    }

    pub async fn create(
        db_pool: &SqlitePool,
        filename: &str,
        uploader_id: i64,
    ) -> Result<Self, sqlx::Error> {
        let file_id =
            sqlx::query(r#"INSERT INTO files (filename, uploader_id) VALUES ($1, $2)"#)
                .bind(filename)
                .bind(uploader_id)
                .execute(db_pool)
                .await?
                .last_insert_rowid();

        Ok(File::from_id(db_pool, file_id).await.unwrap())
    }
}

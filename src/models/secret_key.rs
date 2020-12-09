use openssl::rand::rand_bytes;
use sqlx::{sqlite::SqliteRow, FromRow, Row, SqlitePool};

#[derive(FromRow)]
struct SecretKey {
    key: Vec<u8>,
}

pub async fn get_or_create(db_pool: &SqlitePool) -> Vec<u8> {
    let result = sqlx::query(r#"SELECT key FROM secret_key LIMIT 1"#)
        .map(|row: SqliteRow| SecretKey { key: row.get(0) })
        .fetch_one(db_pool)
        .await;

    if let Ok(secret_key) = result {
        return secret_key.key;
    }

    // We failed to fetch the key from the database; create a new one
    // here.

    let new_key = generate_new_secret_key();

    sqlx::query(r#"INSERT INTO secret_key (key) VALUES ($1)"#)
        .bind(&new_key)
        .execute(db_pool)
        .await
        .expect("Failed to insert secret key into database.");

    new_key
}

fn generate_new_secret_key() -> Vec<u8> {
    let mut buf = [0; 32];
    rand_bytes(&mut buf).unwrap();
    buf.to_vec()
}

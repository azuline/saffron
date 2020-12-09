use openssl::{memcmp, rand::rand_bytes};
use sodiumoxide::crypto::pwhash::argon2id13;
use sqlx::{sqlite::SqliteRow, FromRow, Row, SqlitePool};

#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub nickname: String,
    pub token_prefix: Vec<u8>,
    pub token_hash: Vec<u8>,
    pub csrf_token: Vec<u8>,
}

impl User {
    pub async fn from_id(db_pool: &SqlitePool, id: i64) -> Option<User> {
        sqlx::query(
            r#"
                SELECT
                    id,
                    nickname,
                    token_prefix,
                    token_hash,
                    csrf_token
                FROM users
                WHERE id = $1
                LIMIT 1
            "#,
        )
        .bind(id)
        .map(|row: SqliteRow| User {
            id: row.get(0),
            nickname: row.get(1),
            token_prefix: row.get(2),
            token_hash: row.get(3),
            csrf_token: row.get(4),
        })
        .fetch_one(db_pool)
        .await
        .ok()
    }

    pub async fn from_token(db_pool: &SqlitePool, token: &[u8]) -> Option<User> {
        let token_prefix = match token.get(0..8) {
            Some(token_prefix) => token_prefix,
            _ => return None,
        };

        let result = sqlx::query(
            r#"
                SELECT
                    id,
                    nickname,
                    token_prefix,
                    token_hash,
                    csrf_token
                FROM users
                WHERE token_prefix = $1
                LIMIT 1
            "#,
        )
        .bind(token_prefix)
        .map(|row: SqliteRow| User {
            id: row.get(0),
            nickname: row.get(1),
            token_prefix: row.get(2),
            token_hash: row.get(3),
            csrf_token: row.get(4),
        })
        .fetch_one(db_pool)
        .await;

        if let Ok(user) = result {
            if verify_token(&user, token) {
                return Some(user);
            }
        }

        None
    }

    pub async fn create(
        db_pool: &SqlitePool,
        nickname: &str,
        token: &[u8; 24],
    ) -> Result<Self, sqlx::Error> {
        let token_prefix = token[..8].to_vec();
        let token_hash = hash_token(&token).to_vec();
        let csrf_token = generate_token().to_vec();

        let user_id = sqlx::query(
            r#"
                INSERT INTO users
                (nickname, token_prefix, token_hash, csrf_token)
                VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(nickname)
        .bind(token_prefix)
        .bind(token_hash)
        .bind(csrf_token)
        .execute(db_pool)
        .await?
        .last_insert_rowid();

        Ok(User::from_id(&db_pool, user_id).await.unwrap())
    }

    pub async fn update_token(
        mut self,
        db_pool: &SqlitePool,
        token: &[u8; 24],
    ) -> Result<Self, sqlx::Error> {
        let token_prefix = &token[..8];
        let token_hash = hash_token(&token).to_vec();

        sqlx::query(
            r#"
                UPDATE users
                SET token_prefix = $1,
                    token_hash = $2
                WHERE id = $3
            "#,
        )
        .bind(token_prefix)
        .bind(&token_hash)
        .bind(self.id)
        .execute(db_pool)
        .await?;

        self.token_prefix = token_prefix.to_vec();
        self.token_hash = token_hash;

        Ok(self)
    }
}

pub fn generate_token() -> [u8; 24] {
    let mut buf = [0; 24];
    rand_bytes(&mut buf).unwrap();
    buf
}

pub fn verify_token(user: &User, token: &[u8]) -> bool {
    match argon2id13::HashedPassword::from_slice(&user.token_hash) {
        Some(hp) => argon2id13::pwhash_verify(&hp, &token),
        _ => false,
    }
}

pub fn check_csrf(user: &User, csrf: &str) -> bool {
    let csrf_token = match hex::decode(&csrf) {
        Ok(token) => token,
        _ => return false,
    };

    if user.csrf_token.len() != csrf_token.len() {
        return false;
    }

    memcmp::eq(&user.csrf_token, &csrf_token)
}

fn hash_token(token: &[u8; 24]) -> [u8; 128] {
    argon2id13::pwhash(
        token,
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap()
    .0
}

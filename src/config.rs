use crate::models::secret_key;
use sqlx::{migrate, sqlite::SqlitePoolOptions, SqlitePool};
use std::{env, fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Config {
    pub upload_dir: PathBuf,
    pub db_pool: SqlitePool,
    pub host_url: String,
    pub secret_key: Vec<u8>,
}

impl Config {
    pub async fn read() -> Self {
        let upload_directory = PathBuf::from(
            env::var("UPLOAD_DIRECTORY")
                .expect("Upload directory not configured in `.env` file."),
        );

        fs::create_dir_all(&upload_directory).unwrap();

        let db_url = env::var("DATABASE_URL")
            .expect("Database URL not configured in `.env` file.");
        touch_database_file(&db_url);

        let db_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();

        let raw_host_url =
            env::var("HOST_URL").expect("Host URL not configured in `.env` file.");

        let host_url = match raw_host_url.strip_suffix("/") {
            Some(host_url) => host_url.to_owned(),
            _ => raw_host_url,
        };

        migrate!("./migrations").run(&db_pool).await.unwrap();

        let secret_key = secret_key::get_or_create(&db_pool).await;

        Self {
            upload_dir: upload_directory,
            db_pool,
            host_url,
            secret_key,
        }
    }
}

// SQLx is a dunce and will error if the file doesn't exist...
fn touch_database_file(db_url: &str) {
    let db_path = PathBuf::from(
        db_url
            .strip_prefix("sqlite://")
            .expect("Invalid DATABASE_URL in `.env` file."),
    );

    if db_path.exists() {
        return;
    }

    // Because SQLX is stupid and won't create a new database.
    fs::File::create(&db_path).unwrap();
}

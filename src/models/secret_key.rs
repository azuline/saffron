use openssl::rand::rand_bytes;
use sqlx::{FromRow, SqlitePool};

#[derive(FromRow)]
struct Secretkey {
    key: Vec<u8>,
}

pub fn get_or_create(db_pool: &SqlitePool) -> Vec<u8> {
    // if let Ok(key) = secret_key::table.first::<SecretKey>(conn) {
    //     return key.key;
    // }

    // let new_key = SecretKey {
    //     key: generate_new_secret_key(),
    // };

    // diesel::insert_into(secret_key::table)
    //     .values(&new_key)
    //     .execute(conn)
    //     .expect("Failed to write secret key to database.");

    // new_key.key
}

fn generate_new_secret_key() -> Vec<u8> {
    let mut buf = [0; 32];
    rand_bytes(&mut buf).unwrap();
    buf.to_vec()
}

use openssl::rand::rand_bytes;
use std::io::Write;
use std::path::Path;
use std::{fs::File, path::PathBuf};

use crate::config::Config;
use crate::models::User;
use actix_multipart::{Field, Multipart};
use actix_web::http::{header, HeaderMap};
use actix_web::{post, web, web::Data, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use sqlx::SqlitePool;

// TODO:
// - JSON response payload.
// - Database persistence.
// - Handle a failed file upload properly, don't leave half-uploaded artifacts on disk...

#[post("/upload")]
pub async fn take_upload(
    request: HttpRequest,
    mut payload: Multipart,
    config: Data<Config>,
) -> Result<HttpResponse, actix_web::Error> {
    // Check the HTTP header for a valid authentication token. If we don't have one, then abort.

    let user = match get_user_from_header(&config.db_pool, &request.headers()).await {
        Some(user) => user,
        _ => return Ok(HttpResponse::Unauthorized().finish()),
    };

    // We need to construct the filepath and return it outside of the
    // chunk handling logic. So we take the first chunk on its own to
    // discover the file extension, and then continue to process
    // the rest.

    let mut filepath = config.upload_dir.clone();
    filepath.push(get_random_basename());

    if let Ok(Some(field)) = payload.try_next().await {
        // Get file extension from first chunk if exists; append it to our
        // filepath.
        let content_type = field.content_disposition().unwrap();
        let filename = Path::new(content_type.get_filename().unwrap());

        if let Some(ext) = filename.extension() {
            filepath.set_extension(ext);
        }

        save_field_to_file(field, filepath.clone()).await?;
    }

    // Iterate over and save the rest of the chunks.
    while let Ok(Some(field)) = payload.try_next().await {
        save_field_to_file(field, filepath.clone()).await?;
    }

    Ok(HttpResponse::Ok().into())
}

async fn get_user_from_header(
    db_pool: &SqlitePool,
    headers: &HeaderMap,
) -> Option<User> {
    let value = headers.get(header::AUTHORIZATION)?.to_str().ok()?;
    let token = hex::decode(value.strip_prefix("Token ")?).ok()?;
    User::from_token(db_pool, &token).await
}

fn get_random_basename() -> String {
    let mut buf: [u8; 8] = [0; 8];
    rand_bytes(&mut buf).unwrap();
    base64::encode_config(buf, base64::URL_SAFE_NO_PAD)
}

async fn save_field_to_file<'a>(
    mut field: Field,
    filepath: PathBuf,
) -> Result<(), actix_web::Error> {
    dbg!(&filepath);
    let mut f = web::block(|| File::create(filepath)).await.unwrap();

    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();

        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }

    Ok(())
}

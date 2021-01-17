use openssl::rand::rand_bytes;
use serde_json::json;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::models::{File, User};
use actix_multipart::{Field, Multipart};
use actix_web::http::{header, HeaderMap};
use actix_web::{post, web, web::Data, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use sqlx::SqlitePool;

#[post("/upload")]
pub async fn take_upload(
    request: HttpRequest,
    mut payload: Multipart,
    config: Data<Config>,
) -> Result<HttpResponse, actix_web::Error> {
    // Check the HTTP header for a valid authentication token. If we don't
    // have one, then abort.
    let user = match get_user_from_header(&config.db_pool, &request.headers()).await {
        Some(user) => user,
        _ => return Ok(HttpResponse::Unauthorized().finish()),
    };

    // We need to construct the filepath and return it outside of the
    // chunk handling logic. So we take the first chunk on its own to
    // discover the file extension, and then continue to process
    // the rest.

    // TODO: If this file upload fails, can we remove the existing written
    // shit from disk? This means not using ? but explicitly catching
    // and wiping file from disk.

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

    // Now, create the file row in database and get our ID, in preparation
    // to return a success JSON to the client.

    let filename = filepath.file_name().unwrap().to_str().unwrap();

    let file = match File::create(&config.db_pool, &filename, user.id).await {
        Ok(file) => file,
        _ => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let response = json!({
        "image_url": format!("{}/f/{}", &config.host_url, &filename),
        "deletion_url": format!("{}/delete/{}", &config.host_url, file.id),
    });

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response.to_string()))
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
    let mut f = web::block(|| std::fs::File::create(filepath))
        .await
        .unwrap();

    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();

        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }

    Ok(())
}

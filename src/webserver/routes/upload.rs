use openssl::rand::rand_bytes;
use std::io::Write;
use std::path::Path;
use std::{fs::File, path::PathBuf};

use crate::config::Config;
use actix_multipart::{Field, Multipart};
use actix_web::{post, web, web::Data, HttpResponse};
use futures::{StreamExt, TryStreamExt};

// TODO:
// - JSON response payload.
// - Database persistence.
// - Authentication via token header.

#[post("/upload")]
pub async fn take_upload(
    mut payload: Multipart,
    config: Data<Config>,
) -> Result<HttpResponse, actix_web::Error> {
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

fn get_random_basename() -> String {
    let mut buf: [u8; 12] = [0; 12];
    rand_bytes(&mut buf).unwrap();
    base64::encode(buf)
}

async fn save_field_to_file<'a>(
    mut field: Field,
    filepath: PathBuf,
) -> Result<(), actix_web::Error> {
    let mut f = web::block(|| File::create(filepath)).await.unwrap();

    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();

        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }

    Ok(())
}

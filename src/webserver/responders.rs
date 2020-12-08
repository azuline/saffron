use actix_web::{Error, HttpResponse, Responder, HttpRequest};
use futures::future::{ready, Ready};
use serde::Serialize;
use tera::Tera;

#[derive(Serialize)]
pub struct Template<T>(&'static str, T);

impl<T> Responder for Template<T> {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let tera = Tera::new(self.0);
    }
}

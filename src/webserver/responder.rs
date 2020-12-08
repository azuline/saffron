use actix_web::error::ErrorInternalServerError;
use actix_web::web::Data;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use tera::{Context, Tera};

pub struct Template(pub &'static str, pub Context);

impl Responder for Template {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let tmpl = req.app_data::<Data<Tera>>().unwrap();

        let response = match tmpl.render(self.0, &self.1) {
            Ok(html) => Ok(HttpResponse::Ok().content_type("text/html").body(html)),
            _ => Err(ErrorInternalServerError("Template error.")),
        };

        ready(response)
    }
}

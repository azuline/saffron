// use crate::webserver::responder::Template;
// use actix_web::dev::{Body, ServiceResponse};
// use actix_web::middleware::errhandlers::ErrorHandlerResponse;
// use actix_web::web::Data;
// use actix_web::Error;
// use tera::{Context, Tera};

//// TODO: This shit doesn't work! What the fuck do they mean by:
////
//// > You can use the ErrorHandlers::handler() method to register a
//// > custom error handler for a specific status code. You can modify
//// > an existing response or create a completly new one. The
//// > error handler can return a response immediately or return a
//// > future that resolves into a > response.
////
//// Clearly, I can't return shit new error!!

// pub fn error_404<B>(
//    mut res: ServiceResponse<B>,
//) -> Result<ErrorHandlerResponse<Body>, Error> {
//    let mut context = Context::new();
//    context.insert("message", "you are lost");
//    let tmpl = res.request().app_data::<Data<Tera>>().unwrap();
//    res.into_response(Template("error.html",
// context).render(tmpl).unwrap());

//    Ok(res
//    // Ok(ErrorHandlerResponse::Response(res))
//}

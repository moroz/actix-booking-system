use actix_web::web::get;
use actix_web::Scope;
use actix_web::{web, Responder};

async fn hello() -> impl Responder {
    format!("Hello world!")
}

pub fn routes() -> Scope {
    web::scope("/").route("/hello", get().to(hello))
}

#[macro_use]
extern crate diesel;

use crate::api::schema as api_schema;
use crate::db::pool;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, HttpServer};
use std::env;

mod api;
mod db;
mod schema;
mod yoyaku_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::init();
    let pool = pool::init_pool();

    let schema = std::sync::Arc::new(api_schema::create_schema());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(schema.clone())
            .wrap(Logger::default())
            .service(
                web::resource("/graphql")
                    .route(web::post().to(api_schema::handle_request))
                    .route(web::get().to(api_schema::graphiql)),
            )
            .service(yoyaku_web::router::routes())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

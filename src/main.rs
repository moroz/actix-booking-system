#[macro_use]
extern crate diesel;
#[macro_use]
extern crate actix_web;

use crate::db::pool;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use std::env;

mod db;
mod yoyaku_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::init();
    let pool = pool::init_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .service(yoyaku_web::router::routes())
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}

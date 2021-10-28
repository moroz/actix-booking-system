#[macro_use]
extern crate diesel;
#[macro_use]
extern crate actix_web;

use crate::db::pool;
use actix_web::{App, HttpServer};

mod db;
mod yoyaku_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = pool::init_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(yoyaku_web::router::routes())
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}

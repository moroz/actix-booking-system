use super::context::Context;
use super::MutationRoot;
use super::QueryRoot;
use crate::db::pool::DbPool;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Result;
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use juniper::{EmptySubscription, RootNode};
use std::sync::Arc;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub async fn handle_request(
    pool: DbPool,
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse> {
    let ctx = Context {
        dbpool: pool.get_ref().to_owned(),
    };
    let res = web::block(move || {
        let result = data.execute_sync(&st, &ctx);
        serde_json::to_string(&result)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

pub async fn graphiql() -> HttpResponse {
    let html = playground_source("http://localhost:8080/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

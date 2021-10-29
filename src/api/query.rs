use super::Context;
use crate::db::models::user::User;
use juniper::{graphql_object, FieldResult, ID};

pub struct QueryRoot;

pub fn parse_id(id: ID) -> i32 {
    let str = id.to_string();
    str.parse().unwrap()
}

#[graphql_object(context = Context)]
impl QueryRoot {
    pub fn apiVersion() -> &'static str {
        "1.0"
    }

    pub fn user(db: &Context, id: ID) -> FieldResult<Option<User>> {
        let user = User::find(&db.dbpool, parse_id(id)).ok();
        Ok(user)
    }
}

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

    pub fn user(db: &Context, id: ID) -> FieldResult<User> {
        let id = parse_id(id);
        let user = User::find(&db.dbpool, id)?;
        Ok(user)
    }
}

use super::Context;
use crate::db::models::{Slot, User};
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

    pub fn user(db: &Context, id: ID) -> Option<User> {
        User::find(&db.dbpool, parse_id(id)).ok()
    }

    pub fn users(db: &Context) -> FieldResult<Vec<User>> {
        let res = User::all(&db.dbpool)?;
        Ok(res)
    }

    pub fn slot(db: &Context, id: ID) -> Option<Slot> {
        Slot::find(&db.dbpool, parse_id(id)).ok()
    }

    pub fn slots(db: &Context) -> FieldResult<Vec<Slot>> {
        let res = Slot::all(&db.dbpool)?;
        Ok(res)
    }
}

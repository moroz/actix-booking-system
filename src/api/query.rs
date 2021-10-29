use crate::db::{models::user::User, pool::DbContext};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode, ID};

pub struct QueryRoot;

pub fn parse_id(id: ID) -> i32 {
    let str = id.to_string();
    str.parse().unwrap()
}

#[graphql_object(context = DbContext)]
impl QueryRoot {
    pub fn apiVersion() -> &'static str {
        "1.0"
    }

    pub async fn user(db: &DbContext, id: ID) -> FieldResult<User> {
        let id = parse_id(id);
        let user = User::find(&db, id)?;
        Ok(user)
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

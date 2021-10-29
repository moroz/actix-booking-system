use crate::db::{models::user::User, pool::DbContext};
use juniper::{graphql_object, FieldResult};

pub struct Context {
    pub db: DbContext,
}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    pub fn apiVersion() -> &'static str {
        "1.0"
    }

    pub async fn user(context: &Context) -> FieldResult<User> {
        let user = User::find(&context.db, 1)?;
        Ok(user)
    }
}

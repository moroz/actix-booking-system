use crate::db::DbContext;
use crate::db::UsersRole;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

pub type DbQueryResult<T> = Result<T, diesel::result::Error>;

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UsersRole,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl User {
    pub fn find(pool: &DbContext, user_id: i32) -> DbQueryResult<User> {
        let conn = pool.get().unwrap();
        users.find(user_id).get_result::<User>(&conn)
    }
}

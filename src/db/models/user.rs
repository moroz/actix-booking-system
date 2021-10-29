use crate::db::Pool;
use crate::db::UsersRole;
use crate::schema::users as users_table;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel_citext::types::CiString;
use juniper::{GraphQLInputObject, GraphQLObject};
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

#[derive(Debug, Serialize, Deserialize, GraphQLInputObject)]
pub struct UserParams {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users_table"]
pub struct NewUser {
    pub email: CiString,
    pub first_name: String,
    pub last_name: String,
}

impl From<&UserParams> for NewUser {
    fn from(params: &UserParams) -> Self {
        Self {
            email: CiString::from(&params.email[..]),
            first_name: params.first_name.clone(),
            last_name: params.last_name.clone(),
        }
    }
}

impl User {
    pub fn find(pool: &Pool, user_id: i32) -> DbQueryResult<User> {
        let conn = pool.get().unwrap();
        users.find(user_id).get_result::<User>(&conn)
    }

    pub fn all(pool: &Pool) -> DbQueryResult<Vec<User>> {
        let conn = pool.get().unwrap();
        users.load::<User>(&conn)
    }
}

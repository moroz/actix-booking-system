use crate::db::DbQueryResult;
use crate::db::Pool;
use crate::db::UsersRole;
use crate::schema::users as users_table;
use crate::schema::users::dsl::*;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel_citext::types::CiString;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UsersRole,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,

    #[graphql(skip)]
    pub password_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, GraphQLInputObject)]
pub struct UserParams {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users_table"]
pub struct NewUser {
    pub email: CiString,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
}

#[derive(Serialize, GraphQLObject, Debug)]
pub struct ValidationError {
    pub key: String,
    pub msg: String,
}

pub enum ValidationResult {
    Ok,
    Error(Vec<ValidationError>),
}

impl UserParams {
    pub fn validate(&self) -> ValidationResult {
        if self.password == self.password_confirmation {
            return ValidationResult::Ok;
        } else {
            return ValidationResult::Error(vec![ValidationError {
                key: format!("password"),
                msg: format!("Passwords don't match."),
            }]);
        }
    }
}

impl From<diesel::result::Error> for ValidationError {
    fn from(err: diesel::result::Error) -> Self {
        ValidationError {
            key: format!("error"),
            msg: format!("{}", err),
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

    pub fn create<'a>(pool: &Pool, params: &UserParams) -> Result<User, Vec<ValidationError>> {
        let conn = pool.get().unwrap();
        match params.validate() {
            ValidationResult::Ok => {
                let new_user = NewUser {
                    password_hash: bcrypt::hash(&params.password, bcrypt::DEFAULT_COST).unwrap(),
                    email: CiString::from(&params.email[..]),
                    first_name: params.first_name.clone(),
                    last_name: params.last_name.clone(),
                };
                match insert_into(users).values(new_user).get_result(&conn) {
                    Ok(new_user) => Ok(new_user),
                    Err(err) => Err(vec![err.into()]),
                }
            }
            ValidationResult::Error(errors) => Err(errors),
        }
    }
}

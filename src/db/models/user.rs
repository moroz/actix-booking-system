use crate::db::DbQueryResult;
use crate::db::Pool;
use crate::db::UsersRole;
use crate::db::{ValidationError, ValidationResult};
use crate::schema::users as users_table;
use crate::schema::users::dsl::*;
use diesel::dsl::{exists, insert_into, select};
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

impl UserParams {
    pub fn validate(&self, conn: &PgConnection) -> ValidationResult {
        let mut errors = ValidationResult::new();
        if self.password != self.password_confirmation {
            errors.add_error("password", "Passwords don't match");
        }
        if email_taken(conn, &self.email[..]) {
            errors.add_error("email", "Has already been taken.");
        }
        errors
    }
}

fn email_taken(conn: &PgConnection, user_email: &str) -> bool {
    select(exists(users.filter(email.eq(CiString::from(user_email)))))
        .get_result(conn)
        .unwrap()
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
        let mut validation_result = params.validate(&conn);
        if validation_result.valid() {
            let new_user = NewUser {
                password_hash: bcrypt::hash(&params.password, bcrypt::DEFAULT_COST).unwrap(),
                email: CiString::from(&params.email[..]),
                first_name: params.first_name.clone(),
                last_name: params.last_name.clone(),
            };
            match insert_into(users).values(new_user).get_result(&conn) {
                Ok(new_user) => return Ok(new_user),
                Err(err) => {
                    validation_result.errors.push(err.into());
                    Err(validation_result.errors)
                }
            }
        } else {
            Err(validation_result.errors)
        }
    }
}

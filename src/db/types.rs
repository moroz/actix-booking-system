use diesel_derive_enum::DbEnum;
use juniper::GraphQLEnum;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Debug, Serialize, Deserialize, GraphQLEnum)]
#[DieselType = "Users_role"]
pub enum UsersRole {
    Customer,
    Admin,
}

pub type DbQueryResult<T> = Result<T, diesel::result::Error>;

use chrono::{DateTime, Utc};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Debug, Serialize, Deserialize)]
#[DieselType = "Users_role"]
pub enum UsersRole {
    Customer,
    Admin,
}

pub type UtcDateTime = DateTime<Utc>;

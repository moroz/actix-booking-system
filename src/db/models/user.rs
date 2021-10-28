use crate::db::UsersRole;
use crate::db::UtcDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    role: UsersRole,
    created_at: UtcDateTime,
    updated_at: UtcDateTime,
}

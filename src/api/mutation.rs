use super::Context;
use crate::db::models::slot::{NewSlot, Slot};
use crate::db::models::user::{User, UserParams};
use crate::db::ValidationError;
use juniper::graphql_object;
use juniper::GraphQLObject;
use serde::Serialize;

pub struct MutationRoot;

#[derive(Debug, Serialize, GraphQLObject)]
pub struct UserMutationResult {
    success: bool,
    errors: Option<Vec<ValidationError>>,
    data: Option<User>,
}

#[derive(Debug, Serialize, GraphQLObject)]
pub struct SlotMutationResult {
    success: bool,
    errors: Option<String>,
    data: Option<Slot>,
}

#[graphql_object(context = Context)]
impl MutationRoot {
    pub fn createUser(db: &Context, params: UserParams) -> UserMutationResult {
        let res = User::create(&db.dbpool, &params);
        match res {
            Ok(user) => UserMutationResult {
                success: true,
                errors: None,
                data: Some(user),
            },
            Err(any) => {
                dbg!(&any);
                UserMutationResult {
                    success: false,
                    errors: Some(any),
                    data: None,
                }
            }
        }
    }

    pub fn createSlot(db: &Context, params: NewSlot) -> SlotMutationResult {
        let res = Slot::create(&db.dbpool, &params);
        match res {
            Ok(slot) => SlotMutationResult {
                success: true,
                errors: None,
                data: Some(slot),
            },
            Err(any) => {
                dbg!(&any);
                SlotMutationResult {
                    success: false,
                    errors: Some(any.to_string()),
                    data: None,
                }
            }
        }
    }
}

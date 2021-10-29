use super::Context;
use crate::db::models::user::{NewUser, User, UserParams};
use juniper::graphql_object;
use juniper::GraphQLObject;
use serde::Serialize;

pub struct MutationRoot;

#[derive(Debug, Serialize, GraphQLObject)]
pub struct UserMutationResult {
    success: bool,
    errors: Option<String>,
    data: Option<User>,
}

#[graphql_object(context = Context)]
impl MutationRoot {
    pub fn createUser(db: &Context, params: UserParams) -> UserMutationResult {
        let res = User::create(&db.dbpool, &NewUser::from(&params));
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
                    errors: Some(any.to_string()),
                    data: None,
                }
            }
        }
    }
}

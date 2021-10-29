use crate::db::Pool;

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

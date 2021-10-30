use crate::db::DbQueryResult;
use crate::db::Pool;
use crate::schema::slots as slots_table;
use crate::schema::slots::dsl::*;
use chrono::prelude::*;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, GraphQLObject, Serialize)]
pub struct Slot {
    id: i32,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable, GraphQLInputObject)]
#[table_name = "slots_table"]
#[graphql(name = "SlotParams")]
pub struct NewSlot {
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
}

impl Slot {
    pub fn find(pool: &Pool, slot_id: i32) -> DbQueryResult<Slot> {
        let conn = pool.get().unwrap();
        slots.find(slot_id).get_result::<Slot>(&conn)
    }

    pub fn all(pool: &Pool) -> DbQueryResult<Vec<Slot>> {
        let conn = pool.get().unwrap();
        slots.load::<Slot>(&conn)
    }

    pub fn create(pool: &Pool, params: &NewSlot) -> DbQueryResult<Slot> {
        let conn = pool.get().unwrap();
        let res = insert_into(slots).values(params).get_result(&conn)?;
        Ok(res)
    }
}

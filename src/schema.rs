#![allow(unused_imports)]

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::db::types::*;

    slots (id) {
        id -> Int4,
        start_time -> Timestamp,
        end_time -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::db::types::*;

    users (id) {
        id -> Int4,
        email -> Citext,
        first_name -> Text,
        last_name -> Text,
        role -> Users_role,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    slots,
    users,
);

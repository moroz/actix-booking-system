-- This file should undo anything in `up.sql`

alter table users
drop password_hash;

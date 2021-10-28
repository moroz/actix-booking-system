-- This file should undo anything in `up.sql`
drop table users;
drop type users_role;
drop extension if exists citext;

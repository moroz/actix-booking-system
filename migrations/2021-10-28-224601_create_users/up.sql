-- Your SQL goes here
create extension if not exists citext with schema public;

create type users_role as enum('customer', 'admin');

create table users (
  id serial primary key,
  email citext not null unique,
  first_name text not null,
  last_name text not null,
  role users_role not null default 'customer',
  created_at timestamp(0) not null default (now() at time zone 'utc'),
  updated_at timestamp(0) not null default (now() at time zone 'utc')
);

select diesel_manage_updated_at('users');

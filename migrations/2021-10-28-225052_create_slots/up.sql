-- Your SQL goes here
create extension if not exists btree_gist with schema public;

create table slots (
  id serial primary key,
  start_time timestamp(0) not null,
  end_time timestamp(0) not null,
  created_at timestamp(0) not null default (now() at time zone 'utc'),
  updated_at timestamp(0) not null default (now() at time zone 'utc'),
  check (end_time > start_time),
  exclude using gist (tsrange(start_time, end_time) with &&)
);

select diesel_manage_updated_at('slots');

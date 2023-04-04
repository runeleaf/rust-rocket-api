-- Add up migration script here
create table if not exists users (
  id serial primary key,
  name varchar(255) not null default 'no name',
  age int not null default 0,
  created_at timestamp not null,
  updated_at timestamp not null
);

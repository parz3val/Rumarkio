-- Add migration script here
create extension if not exists "uuid-ossp";

drop type if exists user_status;

create type user_status as enum ('verified', 'registered', 'deleted');

create table
  if not exists users (
    id uuid primary key,
    email varchar(255) unique not null,
    password_hash varchar(1024) not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    status user_status not null default 'registered'
);

create unique index user_email_index on users (email);

-- Add migration script here
create type gender as enum (
  'male',
  'female',
  'lesbian',
  'gay',
  'bisexual',
  'trans',
  'queer',
  'pan',
  'other',
  'mind_your_business'
);

create table if not exists users_profile(
  id uuid not null primary key,
  user_id uuid not null,
  profile_picture varchar(1024),
  timezone varchar(255) not null default 'utc',
  first_name varchar(255) not null default '',
  user_gender gender not null default 'bisexual',
  language varchar(10) not null default 'en',
  last_name varchar(255) not null default '',
  date_of_birth date,
  phone_number varchar(20),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  constraint user_id_fk foreign key (user_id) references users(id) on delete cascade
);

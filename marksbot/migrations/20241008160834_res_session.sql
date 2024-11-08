-- Add migration script here
create table if not exists user_sessions(
  id uuid not null primary key,
  user_id uuid not null,
  image_url varchar(1024),
  title varchar(1024),
  description text,
  reason varchar(255) not null default 'web-browsing',
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  constraint user_id_fk foreign key (user_id) references users(id) on delete cascade
);

create index session_title_index on user_sessions (title);
create unique index unique_session_title_user_id_index on user_sessions (title, user_id);

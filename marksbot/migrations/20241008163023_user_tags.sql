-- Add migration script here
create table if not exists user_tags(
  id uuid not null primary key,
  user_id uuid not null,
  title varchar(1024) not null default 'default',
  description text,
  image_url varchar(1024),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  constraint user_id_fk foreign key (user_id) references users(id) on delete cascade
);

create index user_tags_title_index on user_tags (title);
create unique index unique_tags_title_user_id_index on user_tags (title, user_id);

-- Add migration script here
create table if not exists user_collections(
  id uuid not null primary key,
  user_id uuid not null,
  image_url varchar(1024),
  title varchar(1024),
  description text,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  constraint user_id_fk foreign key (user_id) references users(id) on delete cascade
);

create index user_collections_title_index on user_collections (title);
create unique index unique_collection_title_user_id_index on user_collections(title, user_id);

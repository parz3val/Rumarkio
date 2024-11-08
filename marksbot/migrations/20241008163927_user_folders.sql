-- Add migration script here
create table if not exists user_folders(
  id uuid not null primary key,
  user_id uuid not null,
  title varchar(1024),
  description text,
  image_url varchar(1024),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  constraint user_id_fk foreign key (user_id) references users(id) on delete cascade
);

create index user_folders_title_index on user_folders (title);
create unique index unique_folders_title_user_id_index on user_folders(title, user_id);

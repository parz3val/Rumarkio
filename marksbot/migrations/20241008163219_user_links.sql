create table if not exists user_links(
  id uuid not null primary key,
  url varchar(1024) not null,
  user_id uuid not null,
  title varchar(1024),
  description text,
  image_url varchar(1024),
  content text,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  constraint user_id_fk foreign key (user_id) references users(id) on delete cascade
);
create index user_title_index on user_links (title);

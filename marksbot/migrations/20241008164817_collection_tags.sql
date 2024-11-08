-- a join table for the collection tags to hold tag id and collections id
-- this is to make sure that collections can have the same or different tags
create table
  if not exists tags_collections (
    id uuid not null primary key,
    collections_id uuid not null,
    tag_id uuid not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    constraint user_collections_fk foreign key (collections_id) references user_collections (id) on delete cascade,
    constraint user_tags_fk foreign key (tag_id) references user_tags (id) on delete cascade
  );
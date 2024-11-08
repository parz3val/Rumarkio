-- Add migration script here
create table
  if not exists links_collections (
    id uuid not null primary key,
    collections_id uuid not null,
    links_id uuid not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    constraint user_collections_fk foreign key (collections_id) references user_collections (id) on delete cascade,
    constraint user_links_fk foreign key (links_id) references user_links (id) on delete cascade
  );
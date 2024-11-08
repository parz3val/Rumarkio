-- a join table to hold tag-id and links-id  when tag is attached to a link
-- this is to make sure the link can have multiple tags 
create table if not exists links_tags(
  id uuid not null primary key,
  links_id uuid not null,
  tag_id uuid not null,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  constraint user_links_fk foreign key (links_id) references user_links(id) on delete cascade,
  constraint user_tags_fk foreign key (tag_id) references user_tags(id) on delete cascade
);

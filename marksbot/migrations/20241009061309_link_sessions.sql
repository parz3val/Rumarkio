-- Table to hold the relationship between the 
-- links and the collections
create table if not exists link_sessions(
  id uuid not null primary key,
  link_id uuid not null,
  session_id uuid not null,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  constraint link_id_fk foreign key (link_id) references user_links(id) on delete cascade,
  constraint session_id_fk foreign key (session_id) references user_sessions(id) on delete cascade
);

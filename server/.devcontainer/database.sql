-- // users table
create table users(
id uuid not null unique,
name varchar(64) not null, 
email varchar(64) not null unique,
password text not null,
created_on timestamp not null default current_timestamp,
primary key (id) );

-- Libraries table
create table libraries(
id uuid not null unique,
name varchar(64) not null, 
created_on timestamp not null default current_timestamp,
modified_on timestamp not null default current_timestamp,
customer_id uuid not null references users(id)
);

-- collections table
create table collections(
id uuid not null unique,
name varchar(64) not null, 
created_on timestamp not null default current_timestamp,
modified_on timestamp not null default current_timestamp,
library uuid references libraries(id),
customer_id uuid not null references users(id)
);

-- tags table
create table tags(
name varchar(64) not null,
id uuid not null unique,
created_on timestamp not null default current_timestamp,
modified_on timestamp not null default current_timestamp,
customer_id uuid not null references users(id)
);

-- bookmarks table
create table bookmarks(
url varchar(1024) not null unique, 
domain varchar(200), 
id uuid not null unique,
created_on timestamp not null default current_timestamp,
modified_on timestamp not null default current_timestamp,
description varchar(1024) not null, 
collection uuid references collections(id),
tag uuid  references tags(id),
customer_id uuid not null references users(id)
);
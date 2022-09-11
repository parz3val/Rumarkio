create table users(
id serial not null unique,
name varchar(64) not null, 
email varchar(64) not null,
password text not null,
primary key (id) );

create table libraries(
id serial not null unique,
name varchar(64) not null, 
customer_id serial not null unique references users(id)
);
create table collections(
name varchar(64) not null, 
id serial not null unique,
created_on timestamp not null default current_timestamp,
modified_on timestamp not null default current_timestamp,
library serial not null  references libraries(id)
);
create table tags(
name varchar(64) not null, 
id serial not null unique,
created_on timestamp not null default current_timestamp,
modified_on timestamp not null default current_timestamp,
customer_id serial not null references users(id)
);
create table bookmarks(
url varchar(1024) not null, 
domain varchar(200) not null, 
id serial not null unique,
created_on timestamp not null default current_timestamp,
modified_on timestamp not null default current_timestamp,
description varchar(1024) not null, 
collection serial not null  references collections(id),
tag serial not null references tags(id)
);
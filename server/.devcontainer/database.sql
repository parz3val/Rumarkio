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

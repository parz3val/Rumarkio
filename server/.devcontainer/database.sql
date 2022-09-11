create table users(
id serial not null unique,
name varchar(64) not null, 
email varchar(64) not null,
password text not null,
primary key (id) );
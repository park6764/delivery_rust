-- Your SQL goes here

create table customers (
    id serial primary key,
    userName varchar(5) not null,
    pw_hash varchar(10) not null,
    addr varchar(20) not null,
    token varchar
);
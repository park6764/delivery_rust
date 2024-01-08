-- Your SQL goes here
create table owners(
    id serial primary key,
    ownerName varchar(5) not null,
    pw_hash varchar(10) not null,
    token varchar
);
-- Your SQL goes here
create table menus(
    id serial primary key,
    menuName varchar(10) not null,
    price integer not null,
    restId integer not null references restaurants
);
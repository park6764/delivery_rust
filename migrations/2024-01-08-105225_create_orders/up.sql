-- Your SQL goes here
create table orders(
    id serial primary key,
    menus integer[] not null,
    userId integer not null references customers,
    restId integer not null references restaurants,
    orderedAt timestamp not null default now()
);
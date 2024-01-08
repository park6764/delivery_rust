-- Your SQL goes here
create table restaurants(
    id serial primary key,
    ownerId integer not null references owners,
    restName varchar(10) not null,
    totalSales bigint default 0
);
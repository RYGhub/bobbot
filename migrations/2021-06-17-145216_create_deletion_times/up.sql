-- Your SQL goes here

create table deletion_times
(
    guild_id bigint
        constraint deletion_times_pk
            primary key,
    deletion_time int not null
);

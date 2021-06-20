-- Your SQL goes here

create table command_channels
(
    guild_id bigint
        constraint command_channels_pk
            primary key,
    channel_id bigint not null
);

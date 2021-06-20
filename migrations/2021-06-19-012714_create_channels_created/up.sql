-- Your SQL goes here

create table channels_created
(
    guild_id bigint,
    channel_id bigint,
    constraint channels_created_pk
        primary key (guild_id, channel_id)
);


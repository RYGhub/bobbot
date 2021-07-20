-- Your SQL goes here

create table presets
(
    guild_id bigint,
    preset_name varchar,
    preset_data jsonb not null,

    constraint presets_pk
        primary key (guild_id, preset_name)
)
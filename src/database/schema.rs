//! This module is autogenerated by [diesel].

table! {
    channels_created (guild_id, channel_id) {
        guild_id -> Int8,
        channel_id -> Int8,
    }
}

table! {
    command_channels (guild_id) {
        guild_id -> Int8,
        channel_id -> Int8,
    }
}

table! {
    deletion_times (guild_id) {
        guild_id -> Int8,
        deletion_time -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    channels_created,
    command_channels,
    deletion_times,
);
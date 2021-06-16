use serenity::model::prelude::{PermissionOverwriteType, PermissionOverwrite, Permissions, UserId, ChannelCategory, GuildChannel};


/// Create a [PermissionOverwrite] which allows all [Permissions].
pub fn allow_all(kind: PermissionOverwriteType) -> PermissionOverwrite {
    PermissionOverwrite {
        allow: Permissions::from_bits(298845201).unwrap(),
        deny: Permissions::empty(),
        kind,
    }
}


/// Create a [PermissionOverwrite] which denies all [Permissions].
pub fn deny_all(kind: PermissionOverwriteType) -> PermissionOverwrite {
    PermissionOverwrite {
        allow: Permissions::all(),
        deny: Permissions::from_bits(298845201).unwrap(),
        kind,
    }
}


/// Create a [PermissionOverwrite] with no [Permissions].
pub fn empty(kind: PermissionOverwriteType) -> PermissionOverwrite {
    PermissionOverwrite {
        allow: Permissions::empty(),
        deny: Permissions::empty(),
        kind,
    }
}


/// Create a [PermissionOverwrite] which allows all [Permissions] for the given [UserId].
pub fn owner(user_id: UserId) -> PermissionOverwrite {
    allow_all(PermissionOverwriteType::Member(user_id))
}


/// Clone the [PermissionOverwrite]s of a [ChannelCategory].
pub fn clone_from_category(category: &ChannelCategory) -> Vec<PermissionOverwrite> {
    category.permission_overwrites.clone()
}


/// Clone the [PermissionOverwrite]s of a [GuildChannel].
pub fn clone_from_guildchannel(channel: &GuildChannel) -> Vec<PermissionOverwrite> {
    channel.permission_overwrites.clone()
}

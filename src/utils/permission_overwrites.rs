use serenity::model::prelude::{PermissionOverwriteType, PermissionOverwrite, Permissions, UserId};


const ALL_VOICE_PERMISSIONS: u64 = 298845201;


/// Create a [PermissionOverwrite] which allows all [Permissions].
pub fn allow_all(kind: PermissionOverwriteType) -> PermissionOverwrite {
    PermissionOverwrite {
        allow: Permissions::from_bits(ALL_VOICE_PERMISSIONS).unwrap(),
        deny: Permissions::empty(),
        kind,
    }
}


/// Create a [PermissionOverwrite] which denies all [Permissions].
pub fn deny_all(kind: PermissionOverwriteType) -> PermissionOverwrite {
    PermissionOverwrite {
        allow: Permissions::all(),
        deny: Permissions::from_bits(ALL_VOICE_PERMISSIONS).unwrap(),
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

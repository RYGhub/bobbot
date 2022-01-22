use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::errors::*;
use crate::database::models::{PresetData};


const ALL_VOICE_PERMISSIONS: u64 = 298845201;


/// Create a [PermissionOverwrite] which allows all [Permissions].
fn allow_all(kind: PermissionOverwriteType) -> PermissionOverwrite {
    PermissionOverwrite {
        allow: Permissions::from_bits(ALL_VOICE_PERMISSIONS).unwrap(),
        deny: Permissions::empty(),
        kind,
    }
}


/// Create a [PermissionOverwrite] which denies all [Permissions].
fn deny_all(kind: PermissionOverwriteType) -> PermissionOverwrite {
    PermissionOverwrite {
        allow: Permissions::all(),
        deny: Permissions::from_bits(ALL_VOICE_PERMISSIONS).unwrap(),
        kind,
    }
}


/// Create a [PermissionOverwrite] with no [Permissions].
fn empty(kind: PermissionOverwriteType) -> PermissionOverwrite {
    PermissionOverwrite {
        allow: Permissions::empty(),
        deny: Permissions::empty(),
        kind,
    }
}


/// Create a [PermissionOverwrite] which allows all [Permissions] for the given [UserId].
fn owner(user_id: UserId) -> PermissionOverwrite {
    allow_all(PermissionOverwriteType::Member(user_id))
}


pub struct ChannelBuilderPermissionOverwrites {
    own_permow: PermissionOverwrite,
    creator_permow: PermissionOverwrite,
    category_permows: Vec<PermissionOverwrite>,
    preset_permows: Vec<PermissionOverwrite>,
}

impl ChannelBuilderPermissionOverwrites {
    /// Merge all [PermissionOverwrite]s into a single [Vec].
    pub fn merge(self) -> Vec<PermissionOverwrite> {
        let mut current = self;
        let mut result = vec![];

        result.append(&mut current.category_permows);
        result.append(&mut current.preset_permows);
        result.push(current.creator_permow);
        result.push(current.own_permow);

        result
    }

    /// Create a [ChannelBuilderPermissionOverwrites] by creating manually the permission overwrites.
    pub fn build(own_id: UserId, creator_id: UserId, category: Option<ChannelCategory>, preset: Option<PresetData>) -> Self {
        ChannelBuilderPermissionOverwrites {
            own_permow: owner(own_id),
            creator_permow: owner(creator_id),
            category_permows: match category {
                Some(category) => category.permission_overwrites,
                None => vec![],
            },
            preset_permows: match preset {
                Some(preset) => preset.permissions,
                None => vec![],
            },
        }
    }

    /// Create a [ChannelBuilderPermissionOverwrites] by retrieving permission overwrites from some common command structs.
    pub async fn fetch(ctx: &Context, creator: &Member, category: &Option<ChannelCategory>, preset: Option<PresetData>) -> BobResult<Self> {
        let own_id = ctx.cache.current_user().await.id.to_owned();
        let creator_id = creator.user.id.to_owned();
        let category = category.to_owned();
        let preset = preset; // FIXME: This can't be converted into owned for some weird reason?

        Ok(ChannelBuilderPermissionOverwrites::build(own_id, creator_id, category, preset))
    }
}

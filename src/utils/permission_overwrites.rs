use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::cache::Cache;
use serenity::http::Http;


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


struct ChannelBuilderPermissionOverwrites {
    own_permow: PermissionOverwrite,
    creator_permow: PermissionOverwrite,
    category_permows: Vec<PermissionOverwrite>,
    preset_permows: Vec<PermissionOverwrite>,
}

impl ChannelBuilderPermissionOverwrites {
    /// Merge all [PermissionOverwrite]s into a single [Vec].
    fn merge(self) -> Vec<PermissionOverwrite> {
        let mut current = self;
        let mut result = vec![];

        result.append(&mut current.category_permows);
        result.append(&mut current.preset_permows);
        result.push(current.creator_permow);
        result.push(current.own_permow);

        result
    }

    /// Create a [ChannelBuilderPermissionOverwrites] by creating manually the permission overwrites.
    fn build(own_id: UserId, creator_id: UserId, category: Option<ChannelCategory>, preset: Option<()>) -> Self {
        ChannelBuilderPermissionOverwrites {
            own_permow: owner(own_id),
            creator_permow: owner(creator_id),
            category_permows: match category {
                Some(category) => category.permission_overwrites,
                None => vec![],
            },
            preset_permows: vec![],  // TODO
        }
    }

    /// Create a [ChannelBuilderPermissionOverwrites] by retrieving permission overwrites from some common command structs.
    fn fetch(cache: &Cache, member: &Member, category_id: &ChannelId) {}
}


pub async fn get_channel_permows(
) {

}


/// Get the [Vec<PermissionOverwrite>] a created channel should have.
pub async fn get_built_channel_permows(cache: &Cache, category: &Option<ChannelCategory>, creator_id: UserId) -> Vec<PermissionOverwrite> {
    let mut ows = vec![];

    if let Some(c) = category.as_ref() {
        ows.append(c.permission_overwrites.clone().as_mut());
    }
    ows.push(owner(cache.current_user().await.id));
    ows.push(owner(creator_id));

    ows
}

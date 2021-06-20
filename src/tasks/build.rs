//! This module contains a task to build a new channel.

use serenity::model::prelude::{Guild, ChannelCategory, ChannelType, GuildChannel, PermissionOverwrite};
use serenity::prelude::{Context};
use crate::errors::{BobResult, BobCatch, ErrorKind};


/// Build a new channel in the specified [`guild`]([Guild]) with the specified `name`.
///
/// The function optionally accepts a [`category`]([ChannelCategory]) and a `preset_name`:
/// - if a `category` is specified, the channel is created in it and inherits its [PermissionOverwrite]s.
/// - if a `preset_name` is specified, the preset with that name is loaded and used as a template for the channel,
///   inheriting the following properties:
///     - [PermissionOverwrite]s
///     - Bitrate (defaulting to 64 kbps)
///     - User limit (defaulting to None)
///
/// # Returns
///
/// - `Ok(channel)` if the channel creation was successful.
/// - `Err(_)` if something went wrong in the creation of the channel.
///
/// # To do
///
/// Presets aren't loaded yet.
///
pub async fn task_build(ctx: &Context, guild: &Guild, name: &str, category: &Option<ChannelCategory>, preset_name: Option<&str>) -> BobResult<GuildChannel> {
    let permissions: Option<Vec<PermissionOverwrite>> = None;
    let bitrate: Option<u32> = None;
    let limit: Option<u32> = None;

    let created = guild.create_channel(&ctx.http, |c| {
        c.name(name.clone());
        c.kind(ChannelType::Voice);
        if let Some(cat) = category {
            c.category(cat.id.clone());
        }

        c.permissions(permissions.unwrap_or(vec![]));
        c.bitrate(bitrate.unwrap_or(64000));
        if let Some(limit) = limit {
            c.user_limit(limit);
        }

        c
    }).await.bob_catch(ErrorKind::Admin, "Failed to create channel")?;

    Ok(created)
}
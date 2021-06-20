use serenity::model::prelude::{Guild, ChannelCategory, ChannelType, GuildChannel, PermissionOverwrite};
use serenity::prelude::{Context};
use crate::errors::{BobResult, bot_error};


pub async fn task_build(ctx: &Context, guild: &Guild, name: &str, category: Option<&ChannelCategory>, preset_name: Option<&str>) -> BobResult<GuildChannel> {
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
    }).await.map_err(bot_error)?;

    Ok(created)
}
use serenity::model::prelude::{GuildChannel, ChannelCategory, Guild, ChannelId, PermissionOverwrite, ChannelType, VideoQualityMode};
use serenity::http::{Http};
use std::convert::{TryFrom};
use crate::basics::result::{BobResult, BobError};


/// Get the [ChannelCategory] of the given [GuildChannel].
pub async fn get_category(http: &Http, channel: &GuildChannel) -> BobResult<Option<ChannelCategory>> {
    let category = channel.category_id;

    if let None = category {
        return Ok(None)
    }

    let category = category.unwrap().to_channel(&http).await
        .map_err(|_| BobError {msg: "Couldn't retrieve channel info"})?;

    let category = category.category()
        .ok_or(BobError {msg: "Channel wasn't a ChannelCategory"})?;

    Ok(Some(category))
}


/// Get the bitrate of the given [GuildChannel].
pub fn get_bitrate(channel: &GuildChannel) -> BobResult<u32> {
    match channel.bitrate {
        Some(bitrate) =>
            u32::try_from(bitrate.clone())
                .map_err(
                    |_| BobError {msg: "Bitrate was larger than a u32"}
                ),
        None => Err(BobError {msg: "Channel did not have any bitrate"}),
    }
}


/// Get the user limit of the given [GuildChannel].
pub fn get_user_limit(channel: &GuildChannel) -> BobResult<Option<u32>> {
    match channel.user_limit {
        Some(user_limit) =>
            u32::try_from(user_limit.clone()).map_or_else(
                |_| Err(BobError {msg: "User limit was larger than a u32"}),
                |n| Ok(Some(n)),
            ),
        None => Ok(None),
    }
}


/// Get the video quality of the given [GuildChannel].
pub fn get_video_quality(channel: &GuildChannel) -> BobResult<VideoQualityMode> {
    match channel.video_quality_mode {
        Some(quality) => Ok(quality),
        None => Err(BobError {msg: "Channel did not have any video quality"})
    }
}


/// Create and return a GuildChannel.
pub async fn create(
    http: &Http,
    guild: &Guild,
    category_id: Option<ChannelId>,
    name: &str,
    permissions: Vec<PermissionOverwrite>,
    bitrate: u32,
    user_limit: Option<u32>
) -> BobResult<GuildChannel>
{
    guild.create_channel(&http, |c| {
        c.name(name);

        c.kind(ChannelType::Voice);

        if let Some(cat) = category_id {
            c.category(cat);
        }

        c.permissions(permissions);

        c.bitrate(bitrate);

        if let Some(limit) = user_limit {
            c.user_limit(limit);
        }

        c
    }).await.map_err(|_| BobError {msg: "Could not create channel"})
}

use serenity::model::prelude::{GuildChannel, ChannelCategory, Guild, ChannelId, PermissionOverwrite, ChannelType, VideoQualityMode, Channel, Member};
use serenity::cache::{Cache};
use serenity::http::{Http};
use std::convert::{TryFrom};
use crate::basics::result::{BobResult, result_error, option_error};


/// Get the full [GuildChannel] from a [ChannelId].
pub async fn get_guild_channel(http: &Http, channel_id: ChannelId) -> BobResult<GuildChannel> {
    channel_id
        .to_channel(&http)
        .await
        .map_err(|e| result_error(e, "Couldn't retrieve channel info"))?
        .guild()
        .ok_or_else(|| option_error("Couldn't get GuildChannel"))
}


/// Get the [ChannelCategory] of the given [GuildChannel].
pub async fn get_category(http: &Http, channel: &GuildChannel) -> BobResult<Option<ChannelCategory>> {
    let category = channel.category_id;

    if let None = category {
        return Ok(None)
    }

    let category = category.unwrap().to_channel(&http).await
        .map_err(|e| result_error(e, "Couldn't retrieve channel info"))?;

    let category = category.category()
        .ok_or_else(|| option_error("Channel wasn't a ChannelCategory"))?;

    Ok(Some(category))
}


/// Get the bitrate of the given [GuildChannel].
pub fn get_bitrate(channel: &GuildChannel) -> BobResult<u32> {
    match channel.bitrate {
        Some(bitrate) =>
            u32::try_from(bitrate.clone())
                .map_err(
                    |e| result_error(e, "Bitrate was larger than a u32")
                ),
        None => Err(option_error("Channel did not have any bitrate")),
    }
}


/// Get the user limit of the given [GuildChannel].
pub fn get_user_limit(channel: &GuildChannel) -> BobResult<Option<u32>> {
    match channel.user_limit {
        Some(user_limit) =>
            u32::try_from(user_limit.clone()).map_or_else(
                |_| Err(option_error("User limit was larger than a u32")),
                |n| Ok(Some(n)),
            ),
        None => Ok(None),
    }
}


/// Get the video quality of the given [GuildChannel].
pub fn get_video_quality(channel: &GuildChannel) -> VideoQualityMode {
    match channel.video_quality_mode {
        Some(quality) => quality,
        None => VideoQualityMode::Auto,
    }
}


/// Get the members of the given [GuildChannel].
pub async fn get_members(cache: &Cache, channel: &GuildChannel) -> BobResult<Vec<Member>> {
    channel.members(&cache)
        .await
        .map_err(|e| result_error(e, "Could not fetch channel members"))
}


/// Create and return a [GuildChannel].
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

}


/// Destroy a [GuildChannel].
pub async fn destroy(http: &Http, channel: GuildChannel) -> BobResult<Channel> {
    channel.delete(&http)
        .await
        .map_err(|e| result_error(e, "Could not delete channel"))
}

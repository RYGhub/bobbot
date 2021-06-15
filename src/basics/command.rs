use serenity::model::prelude::{Message, Guild, GuildChannel, PermissionOverwrite, ChannelCategory, UserId};
use serenity::client::{Cache};
use serenity::http::{Http};
use crate::basics::result::{BobError, BobResult};
use crate::basics::voice::{get_voice_channel as get_voice_channel_full};
use crate::basics::permows;
use crate::basics::presets::BobPreset;


/// Get the guild the message was sent in.
pub async fn get_guild(cache: &Cache, msg: &Message) -> BobResult<Guild> {
    msg.guild(&cache)
        .await
        .ok_or(BobError{ msg: "Message did not have a Guild" })
}


/// Get the channel the message was sent in.
pub async fn get_channel(cache: &Cache, msg: &Message) -> BobResult<GuildChannel> {
    msg.channel(&cache)
        .await
        .ok_or(BobError{ msg: "Couldn't retrieve channel" })?
        .guild()
        .ok_or(BobError{ msg: "Channel wasn't a GuildChannel" })
}


/// Notify the chat channel that the bot is typing.
pub async fn broadcast_typing(http: &Http, channel: &GuildChannel) -> BobResult<()> {
    channel
        .broadcast_typing(&http)
        .await
        .map_err(|_| BobError{ msg: "Couldn't broadcast typing" })
}


/// Get the voice [GuildChannel] the author of the given message is currently in.
///
/// If the user is not connected to voice, the function will return [None].
pub async fn get_voice_channel(http: &Http, guild: &Guild, msg: &Message) -> BobResult<Option<GuildChannel>> {
    get_voice_channel_full(&http, &guild, &msg.author.id).await
}


/// Get the [Vec<PermissionOverwrite>] a created channel should have.
pub async fn get_permows(cache: &Cache, category: &Option<ChannelCategory>, user_id: UserId) -> Vec<PermissionOverwrite> {
    let mut ows = vec![];

    if let Some(c) = category.as_ref() {
        ows.append(permows::clone_from_category(&c).as_mut());
    }
    ows.push(permows::owner(cache.current_user().await.id));
    ows.push(permows::owner(user_id));

    ows
}


/// Get the [Vec<PermissionOverwrite>] a created channel using a certain preset should have.
pub async fn get_permows_with_preset(cache: &Cache, category: &Option<ChannelCategory>, user_id: UserId, preset: &BobPreset) -> Vec<PermissionOverwrite> {
    let mut ows = get_permows(&cache, &category, user_id).await;
    ows.append(preset.permows.clone().as_mut());
    ows
}

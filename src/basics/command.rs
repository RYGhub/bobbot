use serenity::model::prelude::{Message, Guild, GuildChannel, PermissionOverwrite, ChannelCategory, UserId};
use serenity::client::{Cache};
use serenity::http::{Http};
use crate::basics::result::{BobResult, option_error, result_error};
use crate::basics::voice::{get_voice_channel as get_voice_channel_full};
use crate::basics::permission_overwrites;
use crate::basics::presets::BobPreset;
use serenity::model::id::ChannelId;


/// Get the guild the message was sent in.
pub async fn get_guild(cache: &Cache, msg: &Message) -> BobResult<Guild> {
    msg.guild(&cache)
        .await
        .ok_or_else(|| option_error("Message did not have a Guild"))
}


/// Get the channel the message was sent in.
pub async fn get_channel(cache: &Cache, msg: &Message) -> BobResult<GuildChannel> {
    msg.channel(&cache)
        .await
        .ok_or_else(|| option_error("Couldn't retrieve channel"))?
        .guild()
        .ok_or_else(|| option_error("Channel wasn't a GuildChannel"))
}


/// Notify the chat channel that the bot is typing.
pub async fn broadcast_typing(http: &Http, channel: &GuildChannel) -> BobResult<()> {
    channel
        .broadcast_typing(&http)
        .await
        .map_err(|e| result_error(e, "Couldn't broadcast typing"))
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
        ows.append(permission_overwrites::clone_from_category(&c).as_mut());
    }
    ows.push(permission_overwrites::owner(cache.current_user().await.id));
    ows.push(permission_overwrites::owner(user_id));

    ows
}


/// Get the [Vec<PermissionOverwrite>] a created channel using a certain preset should have.
pub async fn get_permows_with_preset(cache: &Cache, category: &Option<ChannelCategory>, user_id: UserId, preset: &BobPreset) -> Vec<PermissionOverwrite> {
    let mut ows = get_permows(&cache, &category, user_id).await;
    ows.append(preset.permows.clone().as_mut());
    ows
}


/// Reply to a [Message].
pub async fn reply(http: &Http, message: &Message, text: String) -> BobResult<Message> {
    message.reply(&http, &text)
        .await
        .map_err(|e| result_error(e, "Could not send reply"))
}


/// Send a [Message] in a [ChannelId].
pub async fn say(http: &Http, channel: &ChannelId, text: String) -> BobResult<Message> {
    channel.say(&http, &text)
        .await
        .map_err(|e| result_error(e, "Could not send message"))
}

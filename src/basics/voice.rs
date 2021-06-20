use serenity::model::prelude::{Guild, UserId, VoiceState, GuildChannel, ChannelId};
use serenity::http::{Http};
use crate::basics::result::{BobResult, result_error, option_error};


/// Get a reference to the [VoiceState] of an user.
///
/// If the user is not connected to voice, the function will return [None].
pub fn get_voice_state<'a>(guild: &'a Guild, user_id: &UserId) -> Option<&'a VoiceState> {
    guild.voice_states.get(&user_id)
}

/// Get the voice GuildChannel an user is currently in.
///
/// If the user is not connected to voice, the function will return [None].
pub async fn get_voice_channel(http: &Http, guild: &Guild, user_id: &UserId) -> BobResult<Option<GuildChannel>> {
    let voice_state = get_voice_state(&guild, &user_id);

    if let None = &voice_state {
        return Ok(None);
    }

    let channel_id = voice_state.unwrap().channel_id
        .ok_or_else(|| option_error("Couldn't get channel id of user's voice state"))?;

    let channel = channel_id.to_channel(&http).await
        .map_err(|e| result_error(e, "Couldn't get channel information of user's voice state"))?;

    let guild_channel = channel.guild()
        .ok_or_else(|| option_error("Voice state channel wasn't a GuildChannel"))?;

    Ok(Some(guild_channel))
}


/// Move an [UserId] to a voice [ChannelId].
pub async fn move_member(http: &Http, guild: &Guild, user_id: &UserId, channel_id: &ChannelId) -> BobResult<()> {
    guild.move_member(
        &http,
        user_id.clone(),
        channel_id.clone(),
    ).await.map_or_else(
        |_| Err(option_error("Couldn't move member")),
        |_| Ok(()),
    )
}

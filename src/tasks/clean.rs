use std::time::{SystemTime, Duration, UNIX_EPOCH};
use serenity::model::prelude::{VoiceState, ChannelId, GuildChannel, Mentionable, Member};
use serenity::prelude::{Context};
use tokio::time::{sleep};
use crate::errors::{BobResult, bot_error};
use crate::database::models::{WithCommandChannel, WithDeletionTime, MayHaveBeenCreatedByBob};


/// _To be run in a `voice_state_change` event._
///
/// Detect if someone left a voice [GuildChannel] and try to [clean_channel] if there's nobody left inside.
///
/// # Returns
///
/// - `None` if no channel was deleted (for any reason, including an error).
/// - `Some(deleted_channel)` if a channel was deleted.
pub async fn task_clean<'a>(
    ctx: &Context,
    old_vs: &'a Option<VoiceState>,
    new_vs: &VoiceState
)
    -> Option<&'a GuildChannel>
{
    debug!("Running clean task");

    let left_channel = get_left_channel_id(old_vs.as_ref(), &new_vs)
        .await?;
    let left_channel = left_channel.to_channel(&ctx.http)
        .await.map_or(None, |r| Some(r))?;
    let left_channel = left_channel.guild()?;

    clean_channel(&ctx, &left_channel)
        .await.map_or_else(|e| None, |o| Some(&left_channel))
}


/// Given two [VoiceState]s, determine if a channel was left and return its [ChannelId].
async fn get_left_channel_id<'a>(old: Option<&'a VoiceState>, new: &'_ VoiceState) -> Option<&'a ChannelId> {
    let old_channel = &old?.channel_id?;

    if let Some(new_channel) = &new.channel_id {
        if old_channel.eq(new_channel) {
            return None;
        }
    }

    Some(old_channel)
}


/// Get a [Vec] of [Member]s inside a voice [GuildChannel].
async fn get_members_in_channel(ctx: &Context, channel: &GuildChannel) -> BobResult<Vec<Member>> {
    channel.members(&ctx.cache)
        .await
        .map_err(bot_error)
}


/// If the channel was created by Bob, check whether there's someone inside the given [GuildChannel]
/// with [get_members_in_channel], then, if nobody's there, start a countdown of DeletionTime, sending a
/// notification in the CommandChannel of the guild.
///
/// If, at the end of the timeout, nobody is still inside the channel, delete it, then edit the previously sent
/// notification.
///
/// # Returns
///
/// - `Err(e)` if an error is encountered while performing the action.
/// - `Ok(None)` if the channel wasn't deleted.
/// - `Ok(Some(c))` if the channel was deleted.
///
/// # Panics
///
/// If the [SystemTime] is somehow before the [UNIX_EPOCH].
async fn clean_channel<'a>(ctx: &Context, channel: &'a GuildChannel) -> BobResult<Option<&'a GuildChannel>> {
    if !channel.was_created_by_bob() {
        return Ok(None);
    }

    let gid = &channel.guild_id;
    let cc = gid.get_command_channel()??;

    let members_in_channel = get_members_in_channel(&ctx, &channel)??;
    if members_in_channel.len() > 0 {
        return Ok(None);
    }

    let time_current = SystemTime::now();
    let countdown = gid.get_deletion_time()?
        .unwrap_or(Duration::from_secs(60));
    let time_deletion = time_current + countdown;

    let mut message = cc.say(
        &ctx.http,
        format!(
            "🕒 {} will be deleted <t:{}:R> if it will still be empty.",
            &channel.mention(),
            &time_deletion.duration_since(UNIX_EPOCH).expect("Time is before the UNIX Epoch!").as_secs()
        )
    ).await?;

    sleep(countdown).await;

    let members_in_channel = get_members_in_channel(&ctx, &channel)??;
    if members_in_channel.len() > 0 {
        return Ok(None);
    }

    let _ = channel.delete(&ctx.http)
        .await.map_err(bot_error)?;

    message.edit(
        &ctx.http,
        |m| m.content(
                format!(
                "🗑 {} was deleted as it was empty.",
                &channel.name,
            )
        )
    ).await?;

    Ok(Some(channel))
}
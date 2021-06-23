//! This module contains a task to clear empty channels.

use std::time::{SystemTime, Duration, UNIX_EPOCH};
use serenity::model::prelude::{VoiceState, ChannelId, GuildChannel, Mentionable};
use serenity::prelude::{Context};
use tokio::time::{sleep};
use crate::errors::{BobResult, BobCatch, ErrorKind};
use crate::database::models::{WithCommandChannel, WithDeletionTime, MayHaveBeenCreatedByBob};
use crate::extensions::*;


/// _To be run in a `voice_state_change` event._
///
/// Detect if someone left a voice [GuildChannel] and run [task_clean] if there's nobody left inside.
///
/// # Returns
///
/// - `Ok(None)` if no channel was deleted.
/// - `Ok(Some))` if a channel was deleted.
/// - `Err(_)` if an error occurred.
///
pub async fn maybe_clean(
    ctx: &Context,
    old_vs: &Option<VoiceState>,
    new_vs: &VoiceState
)
    -> BobResult<Option<()>>
{
    match &get_left_channel_id(&old_vs, &new_vs).await {
        None => Ok(None),
        Some(c) => {
            let channel = &c.ext_guild_channel(&ctx.http).await?;

            if !channel.was_created_by_bob()? {
                return Ok(None);
            }

            let result = task_clean(&ctx, &channel).await?.map(|_| ());
            Ok(result)
        },
    }
}


/// Given two [VoiceState]s, determine if a channel was left and return its [ChannelId].
async fn get_left_channel_id(old: &Option<VoiceState>, new: &VoiceState) -> Option<ChannelId> {
    let old_channel = old.as_ref().or(None)?.channel_id.or(None)?;

    if let Some(new_channel) = &new.channel_id {
        if old_channel.eq(new_channel) {
            return None;
        }
    }

    Some(old_channel.clone())
}


/// If the channel was created by Bob, check whether there's someone inside the given [GuildChannel],
/// then, if nobody's there, start a countdown of DeletionTime, sending a
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
pub async fn task_clean<'a>(ctx: &'_ Context, channel: &'a GuildChannel) -> BobResult<Option<&'a GuildChannel>> {
    debug!("Running task: clean | #{}", &channel.name);

    let gid = &channel.guild_id;
    let cc = gid.get_command_channel()?
        .bob_catch(ErrorKind::Admin, "No command channel has been set in this Server.")?;

    let members_in_channel = channel.ext_members(&ctx.cache).await?;
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
            &time_deletion.duration_since(UNIX_EPOCH)
                .bob_catch(ErrorKind::Admin, "System time is before the UNIX epoch.")?.as_secs(),
        )
    ).await
        .bob_catch(ErrorKind::Admin, "Couldn't send countdown message.")?;

    sleep(countdown).await;

    let members_in_channel = channel.ext_members(&ctx.cache).await?;
    if members_in_channel.len() > 0 {
        return Ok(None);
    }

    let _ = channel.delete(&ctx.http)
        .await.bob_catch(ErrorKind::Admin, "Couldn't delete channel.")?;

    message.edit(
        &ctx.http,
        |m| m.content(
                format!(
                "🗑 _#{}_ was deleted, as it was empty.",
                &channel.name,
            )
        )
    ).await.bob_catch(ErrorKind::Admin, "Couldn't edit sent message")?;

    Ok(Some(channel))
}
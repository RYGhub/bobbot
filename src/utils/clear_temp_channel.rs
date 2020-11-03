use std::result;
use std::env;

use serenity;
use serenity::prelude::*;
use serenity::model::prelude::*;
use once_cell::sync::Lazy;


/// Check whether an user left a channel and delete temp channels.
pub async fn clear_empty_temp_channel(ctx: Context, guild: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) -> result::Result<(), &'static str> {
    debug!("Fetching guild data");
    let guild = guild.ok_or("Unknown guild_id")?;
    let guild: PartialGuild = guild.to_partial_guild(&ctx.http).await.or(Err("Could not fetch guild data"))?;

    debug!("Getting channel id");
    let old = old.ok_or("User just joined voice chat")?;
    let old_channel = &old.channel_id.ok_or("User was in an unknown channel")?;

    debug!("Ensuring a channel leave happened");
    if let Some(new_channel) = &new.channel_id {
        if old_channel == new_channel {
            return Err("Channel didn't change");
        }
    }

    debug!("Finding the category of the channel that was left");
    let old_channel = old_channel
        .to_channel(&ctx.http).await.or(Err("Could not fetch channel data"))?
        .guild().ok_or("Channel was not in a guild")?;
    let old_channel_category_id = &old_channel.category_id.ok_or("Previous channel isn't in any category")?;

    debug!("Finding bob category");
    static BOB_CHANNEL_NAME: Lazy<String> = Lazy::new(|| {env::var("BOB_CHANNEL_NAME").expect("Missing BOB_CHANNEL_NAME envvar.")});
    let mut bob_channel: Option<&GuildChannel> = None;
    let all_channels = guild.channels(&ctx.http).await.or(Err("Could not fetch guild channels"))?;
    for c in all_channels.values() {
        if c.name == (*BOB_CHANNEL_NAME) {
            bob_channel = Some(c);
            break;
        }
    }
    let bob_channel = bob_channel.ok_or("No bob channel found")?;
    let bob_category_id = &bob_channel.category_id.ok_or("No bob category found")?;

    debug!("Ensuring channel to be deleted is in the bob category");
    if old_channel_category_id != bob_category_id {
        return Err("Channel isn't in the bob category");
    }

    debug!("Checking for manage channel permissions");
    let bot_id = &ctx.cache.current_user_id().await;
    let perms: Permissions = old_channel.permissions_for_user(&ctx.cache, bot_id).await.or(Err("Could not fetch self permissions"))?;

    if !perms.manage_channels() {
        return Err("Missing permissions to delete the channel");
    }

    debug!("Fetching channel members for the first time");
    let members: Vec<Member> = old_channel.members(&ctx.cache).await.or(Err("Could not fetch channel members"))?;

    if members.len() != 0 {
        return Err("Channel isn't empty");
    }

    debug!("Notifying of the channel deletion");
    let _ = bob_channel.say(&ctx.http, format!("🕒 Temp channel <#{}> is empty and will be deleted in 60 seconds, unless someone re-joins.", &old_channel.id)).await.or(Err("Could not send deletion message"));

    debug!("Starting grace time before channel deletion");
    tokio::time::delay_for(std::time::Duration::from_secs(60)).await;

    debug!("Fetching channel members for the second time");
    let members: Vec<Member> = old_channel.members(&ctx.cache).await.or(Err("Could not fetch channel members"))?;

    if members.len() != 0 {
        return Err("Channel isn't empty anymore");
    }

    info!("Deleting #{}", &old_channel.name);
    old_channel.delete(&ctx.http).await.or(Err("Failed to delete channel"))?;

    debug!("Notifying the chat of the channel deletion");
    let _ = bob_channel.say(&ctx.http, format!("🗑 Temp channel <#{}> was deleted, as it stayed empty for 60 seconds.", &old_channel.id)).await.or(Err("Could not send deletion message"));

    Ok(())
}

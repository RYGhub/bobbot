mod checks;
mod commands;
mod utils;


use std::result;
use std::env;

#[macro_use]
extern crate log;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use once_cell::sync::Lazy;

use crate::commands::bob::BOB_GROUP;


struct BobHandler;
impl EventHandler for BobHandler {
    /// Handle the ready event.
    fn ready(&self, _context: Context, ready: Ready) {
        info!("{} is ready!", &ready.user.name);
    }

    /// Called when the voice state of an user changes.
    // IntelliJ Rust inspection is broken
    // https://github.com/intellij-rust/intellij-rust/issues/1191
    // noinspection RsTraitImplementation
    fn voice_state_update(&self, ctx: Context, guild_id: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) {
        debug!("Received a voice state update");

        match clear_empty_temp_channel(ctx, guild_id, old, new) {
            Err(s) => {
                debug!("Not deleting: {}", s);
            }
            _ => (),
        }
    }
}


/// Check whether an user left a channel and delete temp channels.
fn clear_empty_temp_channel(ctx: Context, guild: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) -> result::Result<(), &'static str> {
    let guild = guild.ok_or("Unknown guild_id")?;
    let guild: PartialGuild = guild.to_partial_guild(&ctx.http).or(Err("Could not fetch guild data"))?;

    let old = old.ok_or("User just joined voice chat")?;
    let old_channel = &old.channel_id.ok_or("User was in an unknown channel")?;

    if let Some(new_channel) = &new.channel_id {
        if old_channel == new_channel {
            return Err("Channel didn't change");
        }
    }

    let old_channel = old_channel
        .to_channel(&ctx.http).or(Err("Could not fetch channel data"))?
        .guild().ok_or("Channel was not in a guild")?;
    let old_channel = old_channel.read();
    let old_channel_category_id = &old_channel.category_id.ok_or("Previous channel isn't in any category")?;

    let members: Vec<Member> = old_channel.members(&ctx.cache).or(Err("Could not fetch channel members"))?;

    if members.len() != 0 {
        return Err("Channel isn't empty");
    }

    static BOB_CHANNEL_NAME: Lazy<String> = Lazy::new(|| {env::var("BOB_CHANNEL_NAME").expect("Missing BOB_CHANNEL_NAME envvar.")});

    // Find the bob channel category
    let mut bob_channel: Option<&GuildChannel> = None;
    let all_channels = guild.channels(&ctx.http).or(Err("Could not fetch guild channels"))?;
    for c in all_channels.values() {
        if c.name == (*BOB_CHANNEL_NAME) {
            bob_channel = Some(c);
            break;
        }
    }
    let bob_channel = bob_channel.ok_or("No bob channel found")?;
    let bob_category_id = &bob_channel.category_id.ok_or("No bob category found")?;

    if old_channel_category_id != bob_category_id {
        return Err("Channel isn't in the bob category");
    }

    info!("Deleting #{}", &old_channel.name);
    old_channel.delete(&ctx.http).or(Err("Failed to delete channel"))?;

    let _ = bob_channel.say(&ctx.http, format!("🗑 Temp channel <#{}> was deleted, as it was empty.", &old_channel.id));

    Ok(())
}


/// Handle command errors.
fn on_error(ctx: &mut Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::OnlyForGuilds => {
            debug!("Rejecting command sent outside of a guild");
            let _ = msg.channel_id.say(&ctx.http, "⚠️ This command only works in a guild.");
        }
        DispatchError::CheckFailed(check, reason) => {
            match reason {
                Reason::Log(l) => {
                    error!("Check {} failed: {}", &check, &l);
                },
                Reason::User(u) => {
                    debug!("Check {} failed", &check);
                    let _ = msg.channel_id.say(&ctx.http, format!("⚠️ {}", &u));
                },
                Reason::UserAndLog {user: u, log: l} => {
                    error!("Check {} failed: {}", &check, &l);
                    let _ = msg.channel_id.say(&ctx.http, format!("⚠️ {}", &u));
                }
                _ => {
                    error!("Check {} failed for an unknown reason.", &check);
                }
            }
        }
        _ => {
            warn!("Unmatched error occoured!");
            let _ = msg.channel_id.say(&ctx.http, "☢️ An unhandled error just occoured! It has been logged to the console.");
        }
    }
}

/// Initialize and start the bot.
fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    env::var("BOB_CHANNEL_NAME").expect("Missing BOB_CHANNEL_NAME");

    pretty_env_logger::init();
    debug!("Logger initialized!");

    let mut client = Client::new(&token, BobHandler).expect("Error creating Discord client");
    debug!("Discord client created!");

    client.with_framework(
        StandardFramework::new().configure(
            |c| c
                .prefix("!")
        )
        .group(&BOB_GROUP)
        .on_dispatch_error(on_error)
    );
    debug!("Client framework initialized!");

    client.start_autosharded().expect("Error starting Discord client");
}

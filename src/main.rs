mod checks;
mod commands;
mod utils;


use std::env;

#[macro_use]
extern crate log;

use serenity;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::commands::bob::BOB_GROUP;


struct BobHandler;

#[serenity::async_trait]
impl EventHandler for BobHandler {
    /// Handle the ready event.
    async fn ready(&self, _context: Context, ready: Ready) {
        info!("{} is ready!", &ready.user.name);
    }

    /// Called when the voice state of an user changes.
    // IntelliJ Rust inspection is broken
    // https://github.com/intellij-rust/intellij-rust/issues/1191
    // noinspection RsTraitImplementation
    async fn voice_state_update(&self, ctx: Context, guild_id: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) {
        debug!("Received a voice state update");

        match utils::clear_temp_channel::clear_empty_temp_channel(ctx, guild_id, old, new).await {
            Err(s) => {
                debug!("Not deleting: {}", s);
            }
            _ => (),
        }
    }
}



/// Handle command errors.
#[hook]
async fn on_error(ctx: &Context, msg: &Message, error: DispatchError) {
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
                    let _ = msg.channel_id.say(&ctx.http, format!("⚠️ {}", &u)).await;
                },
                Reason::UserAndLog {user: u, log: l} => {
                    error!("Check {} failed: {}", &check, &l);
                    let _ = msg.channel_id.say(&ctx.http, format!("⚠️ {}", &u)).await;
                }
                _ => {
                    error!("Check {} failed for an unknown reason.", &check);
                }
            }
        }
        _ => {
            warn!("Unmatched error occoured!");
            let _ = msg.channel_id.say(&ctx.http, "☢️ An unhandled error just occoured! It has been logged to the console.").await;
        }
    }
}

/// Initialize and start the bot.
#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    env::var("BOB_CHANNEL_NAME").expect("Missing BOB_CHANNEL_NAME");

    pretty_env_logger::init();
    debug!("Logger initialized!");

    let mut client = Client::builder(&token)
        .event_handler(BobHandler)
        .framework(
            StandardFramework::new().configure(
                |c| c
                    .prefix("!")
            )
                .group(&BOB_GROUP)
                .on_dispatch_error(on_error)
        )
        .await
        .expect("Error creating Discord client");
    debug!("Discord client created!");

    client.start_autosharded().await.expect("Error starting Discord client");
}

extern crate pretty_env_logger;
extern crate dotenv;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel;

mod tasks;
mod checks;
mod database;
mod errors;
mod args;
mod utils;

use std::env;
use serenity;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;
use dotenv::{dotenv};
use crate::tasks::clean::task_clean;


struct BobHandler;

#[serenity::async_trait]
impl EventHandler for BobHandler {

    /// Handle the ready event.
    async fn ready(&self, _context: Context, ready: Ready) {
        info!("{} is ready!", &ready.user.name);
    }

    /// Called when the voice state of an user changes.
    async fn voice_state_update(&self, ctx: Context, gid: Option<GuildId>, old_vs: Option<VoiceState>, new_vs: VoiceState) {
        debug!("Received a VoiceState update: {:?} {:?} {:?}", &gid, &old_vs, &new_vs);

        debug!("Starting clean task");
        match task_clean(&ctx, &gid, &old_vs, &new_vs).await {
            None => debug!("Nothing to clean"),
            Some(_) => debug!("Channel cleaned")
        }
    }
}


/// Handle command errors.
#[hook]
async fn on_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::OnlyForGuilds => {
            debug!("Rejecting command sent outside of a guild");
            let _ = msg.channel_id.say(
                &ctx.http,
                format!("⚠️ This command only works in a server channel."),
            );
        }
        DispatchError::Ratelimited(info) => {
            warn!("Rate limited for {} seconds!", &info.as_secs().to_string());
            let _ = msg.channel_id.say(
                &ctx.http,
                format!("⚠️ The bot is currently rate limited. Try again in {} seconds.", &info.as_secs().to_string()));
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
    pretty_env_logger::init();

    match dotenv() {
        Ok(_) => info!(".env: loaded"),
        Err(_) => info!(".env: not present"),
    }

    let token = env::var("DISCORD_TOKEN")
        .expect("Missing DISCORD_TOKEN");

    let appid = env::var("DISCORD_APPID")
        .expect("Missing DISCORD_APPID");
    let appid = appid.parse::<u64>()
        .expect("Invalid integer DISCORD_APPID");

    let prefix = env::var("BOB_PREFIX")
        .unwrap_or(String::from("!"));

    let mut client = Client::builder(&token)
        .event_handler(BobHandler)
        .application_id(appid)
        .framework(
            StandardFramework::new()
                .configure(|c| c
                    .prefix(&prefix)
                )
                // .group(&BOB_GROUP)
                .on_dispatch_error(on_error)
        )
        .await
        .expect("Error creating Discord client");

    client.start_autosharded()
        .await
        .expect("Error starting Discord client");
}

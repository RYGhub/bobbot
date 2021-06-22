//! This crate contains the source code of Bob, a [Discord](https://discord.com/) bot that handles temporary channels.

#![warn(missing_docs)]

extern crate pretty_env_logger;
extern crate dotenv;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel;

mod tasks;
mod checks;
mod database;
mod errors;
mod utils;
mod extensions;
mod commands;

use std::env;
use serenity::prelude::*;
use serenity::model::prelude::*;
use dotenv::{dotenv};
use crate::errors::*;
use crate::extensions::*;
use crate::tasks::clean::maybe_clean;
use crate::utils::command_router::{route_command_interaction};
use crate::utils::discord_display::DiscordDisplay;


struct BobHandler;

impl BobHandler {
    async fn register_commands(&self, ctx: &Context) -> BobResult<()> {
        ApplicationCommand::create_global_application_command(&ctx.http, |c| c
            .name("build")
            .description("Build a new temporary channel.")
            .create_option(|o| o
                .kind(ApplicationCommandOptionType::String)
                .name("name")
                .description("The name of the channel to build")
                .required(true)
            )
            .create_option(|o| o
                .kind(ApplicationCommandOptionType::String)
                .name("preset")
                .description("The preset to use when building the channel")
                .required(false)
            )
        ).await.bob_catch(ErrorKind::Admin, "Couldn't create global command")?;

        Ok(())
    }
}

#[serenity::async_trait]
impl EventHandler for BobHandler {

    /// Handle the ready event.
    async fn ready(&self, ctx: Context, ready: Ready) {
        debug!("Received event: ready");

        info!("{} is ready!", &ready.user.name);

        /*
        info!("Registering new commands...");
        match self.register_commands(&ctx).await {
            Ok(_) => debug!("Commands registered successfully"),
            Err(_) => warn!("Failed to register commands"),
        }
        */

        match ApplicationCommand::get_global_application_commands(&ctx.http).await {
            Ok(commands) => debug!("Available commands: {:?}", &commands),
            Err(_) => warn!("Failed to get available commands list"),
        };
    }

    /// Called when the voice state of an user changes.
    async fn voice_state_update(&self, ctx: Context, _gid: Option<GuildId>, old_vs: Option<VoiceState>, new_vs: VoiceState) {
        debug!("Received event: voice_state_update");

        match maybe_clean(&ctx, &old_vs, &new_vs).await {
            Err(e) => warn!("{}", e),
            Ok(_) => {},
        };
    }

    /// Called when a new interaction is started.
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        debug!("Received event: interaction_create | {:?}", &interaction);

        match &interaction.data {
            None => {
                if let Err(err) = interaction.pong(&ctx.http).await {
                    warn!("{:?}", &err);
                }
            },
            Some(data) => {
                if let InteractionData::ApplicationCommand(data) = data {
                    let content = match route_command_interaction(&ctx, &interaction, &data).await {
                        Ok(s) => s,
                        Err(e) => e.to_discord(),
                    };

                    let result = interaction.create_interaction_response(&ctx.http, |r| r
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|d| d
                            .content(content)
                        )
                    ).await;

                    if let Err(err) = result {
                        warn!("{:?}", &err);
                    }
                }
                else {
                    warn!("Received unknown interaction data, ignoring");
                }
            }
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

    let mut client = Client::builder(&token)
        .event_handler(BobHandler)
        .application_id(appid)
        .await
        .expect("Error creating Discord client");

    client.start_autosharded()
        .await
        .expect("Error starting Discord client");
}

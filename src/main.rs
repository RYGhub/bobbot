//! This crate contains the source code of Bob, a [Discord](https://discord.com/) bot that handles temporary channels.

#![warn(missing_docs)]

extern crate pretty_env_logger;
extern crate dotenv;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

mod tasks;
mod database;
mod errors;
mod utils;
mod extensions;
mod commands;

use std::env;
use serenity::prelude::*;
use serenity::model::prelude::*;
use dotenv::{dotenv};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::application::command::{Command, CommandOptionType};
use crate::errors::*;
use crate::tasks::clean::{maybe_clean_oc, maybe_clean_vsc};
use crate::utils::command_router::{handle_command_interaction};
use crate::utils::discord_display::DiscordDisplay;
use crate::database::models::{connect as db_connect};


diesel_migrations::embed_migrations!();


struct BobHandler;

impl BobHandler {
    async fn register_commands(&self, ctx: &Context) -> BobResult<()> {
        Command::create_global_application_command(&ctx.http, |c| c
            .name("build")
            .description("Build a new temporary channel.")
            .create_option(|o| o
                .kind(CommandOptionType::String)
                .name("name")
                .description("The name of the channel to build")
                .required(true)
            )
            .create_option(|o| o
                .kind(CommandOptionType::String)
                .name("preset")
                .description("The preset to use to create the channel.")
                .required(false)
            )
            .create_option(|o| o
                .kind(CommandOptionType::String)
                .name("kind")
                .description("The type of channel to create.")
                .required(false)
                .add_string_choice("Voice", "Voice")
                .add_string_choice("Stage", "Stage")
            )
        ).await.bob_catch(ErrorKind::Admin, "Couldn't create global command")?;

        Command::create_global_application_command(&ctx.http, |c| c
            .name("save")
            .description("Save a new preset.")
            .create_option(|o| o
                .kind(CommandOptionType::String)
                .name("preset")
                .description("The name of the preset to create or overwrite.")
                .required(true)
            )
            .create_option(|o| o
                .kind(CommandOptionType::Channel)
                .name("template")
                .description("The channel to base the preset on.")
                .required(true)
            )
            .create_option(|o| o
                .kind(CommandOptionType::Boolean)
                .name("overwrite")
                .description("If a template with the same name already exists, overwrite it?")
                .required(false)
            )
        ).await.bob_catch(ErrorKind::Admin, "Couldn't create global command")?;

        Command::create_global_application_command(&ctx.http, |c| c
            .name("config")
            .description("Configure the bot.")
            .create_option(|o| o
                .kind(CommandOptionType::SubCommand)
                .name("cc")
                .description("Set the channel where the bot should send messages in.")
                .create_sub_option(|so| so
                    .kind(CommandOptionType::Channel)
                    .name("channel")
                    .description("The text channel where the bot should send messages in.")
                    .required(true)
                )
            )
            .create_option(|o| o
                .kind(CommandOptionType::SubCommand)
                .name("dt")
                .description("Set the time before channel deletion.")
                .create_sub_option(|so| so
                    .kind(CommandOptionType::Integer)
                    .name("timeout")
                    .description("The time before channel deletion.")
                    .required(true)
                    .add_int_choice("5 seconds", 5)
                    .add_int_choice("30 seconds", 30)
                    .add_int_choice("1 minute", 60)
                    .add_int_choice("2 minutes", 120)
                    .add_int_choice("5 minutes", 300)
                    .add_int_choice("10 minutes", 600)
                    .add_int_choice("30 minutes", 1800)
                    .add_int_choice("1 hour", 3600)
                    .add_int_choice("3 hours", 7200)
                    .add_int_choice("6 hours", 21600)
                )
            )
        ).await.bob_catch(ErrorKind::Admin, "Couldn't create global command")?;

        Ok(())
    }
}

#[serenity::async_trait]
impl EventHandler for BobHandler {
    /// Called when a new channel is created.
    async fn channel_create(&self, ctx: Context, channel: &GuildChannel) {
        debug!("Received event: channel_create");
        if let Err(e) = maybe_clean_oc(&ctx, channel).await {
            warn!("{}", e)
        }
    }

    /// Handle the ready event.
    async fn ready(&self, ctx: Context, ready: Ready) {
        debug!("Received event: ready");

        info!("{} is ready!", &ready.user.name);

        let register_commands = env::var("DISCORD_REGISTER_COMMANDS").is_ok();
        match register_commands {
            true => {
                info!("Registering new commands, DISCORD_REGISTER_COMMANDS is set...");
                match self.register_commands(&ctx).await {
                    Ok(_) => debug!("Commands registered successfully"),
                    Err(e) => warn!("Failed to register commands: {}", &e),
                }
                info!("New commands registered, they may take up to an hour to appear.")
            }
            false => {
                info!("Not registering commands, DISCORD_REGISTER_COMMANDS is not set.")
            }
        }

        match Command::get_global_application_commands(&ctx.http).await {
            Ok(commands) => debug!("Available commands: {:?}", &commands),
            Err(e) => warn!("Failed to get available commands list: {:?}", &e),
        };
    }

    /// Called when the voice state of an user changes.
    async fn voice_state_update(&self, ctx: Context, old_vs: Option<VoiceState>, new_vs: VoiceState) {
        debug!("Received event: voice_state_update");

        if let Err(e) = maybe_clean_vsc(&ctx, &old_vs, &new_vs).await {
            warn!("{}", e)
        };
    }

    /// Called when a new interaction is started.
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        debug!("Received event: interaction_create | {:?}", &interaction);

        match &interaction {
            Interaction::ApplicationCommand(command) => {
                // Respond early, as not all commands may complete in less than 3 seconds
                let result = command.create_interaction_response(&ctx.http, |r| r
                    .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                ).await;

                if let Err(err) = result {
                    warn!("Could not respond to interaction: {:?}", &err);
                    return;
                }

                let content = match handle_command_interaction(&ctx, command).await {
                    Ok(s) => s,
                    Err(e) => e.to_discord(),
                };

                let result = command.edit_original_interaction_response(&ctx.http, |r| r
                    .content(content)
                ).await;

                if let Err(err) = result {
                    warn!("Could not update interaction response: {:?}", &err);
                    return;
                }
            },
            _ => {
                warn!("Received unknown interaction, ignoring");
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

    let _ = env::var("DATABASE_URL")
        .expect("Missing DATABASE_URL");

    info!("Running migrations...");
    {
        let connection = db_connect();
        embedded_migrations::run(&connection).expect("Could not run embedded migrations");
    }
    info!("Successfully ran all migrations.");


    debug!("Building client...");
    let mut client = Client::builder(&token, GatewayIntents::GUILDS | GatewayIntents::GUILD_VOICE_STATES)
        .event_handler(BobHandler)
        .application_id(appid)
        .await
        .expect("Error creating Discord client");

    info!("Starting Discord client!");
    client.start_autosharded()
        .await
        .expect("Error starting Discord client");
}

//! This crate contains the source code of Bob, a [Discord](https://discord.com/) bot that handles temporary channels.

#![warn(missing_docs)]

extern crate pretty_env_logger;
extern crate dotenv;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

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
use crate::utils::command_router::{handle_command_interaction};
use crate::utils::discord_display::DiscordDisplay;
use crate::database::models::{connect as db_connect, connect};


diesel_migrations::embed_migrations!();


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
                .description("The preset to use to create the channel.")
                .required(false)
            )
        ).await.bob_catch(ErrorKind::Admin, "Couldn't create global command")?;

        ApplicationCommand::create_global_application_command(&ctx.http, |c| c
            .name("save")
            .description("Save a new preset.")
            .create_option(|o| o
                .kind(ApplicationCommandOptionType::String)
                .name("preset")
                .description("The name of the preset to create or overwrite.")
                .required(true)
            )
            .create_option(|o| o
                .kind(ApplicationCommandOptionType::Channel)
                .name("template")
                .description("The channel to base the preset on.")
                .required(true)
            )
            .create_option(|o| o
                .kind(ApplicationCommandOptionType::Boolean)
                .name("overwrite")
                .description("If a template with the same name already exists, overwrite it?")
                .required(false)
            )
        ).await.bob_catch(ErrorKind::Admin, "Couldn't create global command")?;

        ApplicationCommand::create_global_application_command(&ctx.http, |c| c
            .name("config")
            .description("Configure the bot.")
            .create_option(|o| o
                .kind(ApplicationCommandOptionType::SubCommand)
                .name("cc")
                .description("Set the channel where the bot should send messages in.")
                .create_sub_option(|so| so
                    .kind(ApplicationCommandOptionType::Channel)
                    .name("channel")
                    .description("The text channel where the bot should send messages in.")
                    .required(true)
                )
            )
            .create_option(|o| o
                .kind(ApplicationCommandOptionType::SubCommand)
                .name("dt")
                .description("Set the time before channel deletion.")
                .create_sub_option(|so| so
                    .kind(ApplicationCommandOptionType::Integer)
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

        match ApplicationCommand::get_global_application_commands(&ctx.http).await {
            Ok(commands) => debug!("Available commands: {:?}", &commands),
            Err(e) => warn!("Failed to get available commands list: {:?}", &e),
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
                    let content = match handle_command_interaction(&ctx, &interaction, &data).await {
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

    let _ = env::var("DATABASE_URL")
        .expect("Missing DATABASE_URL");

    info!("Running migrations...");
    {
        let connection = db_connect();
        embedded_migrations::run(&connection);
    }
    info!("Successfully ran all migrations.");


    debug!("Building client...");
    let mut client = Client::builder(&token)
        .event_handler(BobHandler)
        .application_id(appid)
        .await
        .expect("Error creating Discord client");

    info!("Starting Discord client!");
    client.start_autosharded()
        .await
        .expect("Error starting Discord client");
}

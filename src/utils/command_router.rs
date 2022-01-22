use serenity::prelude::Context;
use serenity::model::prelude::*;
use serenity::model::interactions::application_command::{ApplicationCommandInteraction, ApplicationCommandInteractionData};
use crate::commands::build::command_build;
use crate::commands::config::{command_config_cc, command_config_dt};
use crate::commands::save::command_save;
use crate::errors::{BobCatch, ErrorKind, BobError, BobResult};


pub async fn handle_command_interaction(ctx: &Context, interaction: &ApplicationCommandInteraction) -> BobResult<String> {
    let guild_id = &interaction.guild_id.as_ref()
        .bob_catch(ErrorKind::Developer, "Interaction has no GuildId")?;

    let channel_id = &interaction.channel_id.as_ref();

    let member = &interaction.member.as_ref()
        .bob_catch(ErrorKind::Developer, "Interaction has no member")?;

    route_command_interaction(&ctx, &guild_id, &channel_id, &member, &interaction.data).await
}


pub async fn route_command_interaction(ctx: &Context, guild_id: &GuildId, channel_id: &ChannelId, member: &Member, data: &ApplicationCommandInteractionData) -> BobResult<String> {
    match data.name.as_str() {
        "build"  => command_build(&ctx, &guild_id, &channel_id, &member, &data).await,
        "save"   => command_save(&ctx, &guild_id, &channel_id, &member, &data).await,
        "config" => route_config(&ctx, &guild_id, &channel_id, &member, &data).await,
        _        => command_invalid().await,
    }
}


pub async fn route_config(ctx: &Context, guild_id: &GuildId, channel_id: &ChannelId, member: &Member, data: &ApplicationCommandInteractionData) -> BobResult<String> {
    let option = data.options.get(0)
        .bob_catch(ErrorKind::Developer, "First interaction option isn't SubCommand")?;

    match option.name.as_str() {
        "cc" => command_config_cc(&ctx, &guild_id, &channel_id, &member, &option.options).await,
        "dt" => command_config_dt(&ctx, &guild_id, &channel_id, &member, &option.options).await,
        _    => command_invalid().await
    }
}


async fn command_invalid() -> BobResult<String> {
    Err(
        BobError::from_msg(ErrorKind::Developer, "Invalid command name")
    )
}
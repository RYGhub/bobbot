use serenity::prelude::Context;
use serenity::model::interactions::{Interaction, ApplicationCommandInteractionData};
use crate::commands::build::command_build;
use crate::errors::{BobCatch, ErrorKind, BobError, BobResult};
use serenity::model::prelude::{GuildId, ChannelId, User};


pub async fn route_command_interaction(ctx: &Context, interaction: &Interaction, data: &ApplicationCommandInteractionData) -> BobResult<String> {
    let guild_id = &interaction.guild_id.as_ref()
        .bob_catch(ErrorKind::Developer, "Interaction has no GuildId")?;

    let channel_id = &interaction.channel_id.as_ref()
        .bob_catch(ErrorKind::Developer, "Interaction has no ChannelId")?;

    let member = &interaction.member.as_ref()
        .bob_catch(ErrorKind::Developer, "Interaction has no member")?;

    match data.name.as_str() {
        "build" => command_build(&ctx, &guild_id, &channel_id, &member, &data).await,
        _       => command_invalid().await,
    }
}


async fn command_invalid() -> BobResult<String> {
    Err(
        BobError::from_msg(ErrorKind::Developer, "Invalid command name")
    )
}
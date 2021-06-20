use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::basics::command::{get_guild, get_channel, broadcast_typing, get_permows_with_preset, reply};
use crate::basics::channel::{get_category, create};
use crate::basics::args::{parse_preset_name, parse_channel_name};
use crate::basics::presets::BobPreset;
use crate::basics::voice::{move_member};


/// Build a new temporary channel with the specified preset.
#[command]
#[aliases("l")]
#[only_in(guilds)]
pub async fn load(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("Running command: !load");

    let guild = get_guild(&ctx.cache, &msg).await?;
    let channel = get_channel(&ctx.cache, &msg).await?;
    let category = get_category(&ctx.http, &channel).await?;
    broadcast_typing(&ctx.http, &channel).await?;

    let preset_name = parse_preset_name(&mut args)?;
    let channel_name = parse_channel_name(args)?;

    let preset = BobPreset::read_guild(&guild.id, &preset_name)?;

    let created = create(
        &ctx.http,
        &guild,
        category.clone().and_then(|cat| Some(cat.id)),  // Can be improved, I think
        &channel_name,
        get_permows_with_preset(&ctx.cache, &category, msg.author.id.clone(), &preset).await,
        preset.bitrate,
        preset.user_limit,
    ).await?;

    move_member(&ctx.http, &guild, &msg.author.id, &created.id).await?;

    reply(
        &ctx.http, &msg,
        format!(
            "🔨 Built channel {} with owner {} from preset `{}`!",
            &created.mention(),
            &msg.author.mention(),
            &preset_name)
    ).await?;

    Ok(())
}

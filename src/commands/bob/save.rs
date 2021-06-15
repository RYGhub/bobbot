use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::basics::command::{get_guild, get_channel, get_voice_channel, broadcast_typing};
use crate::basics::args::{parse_preset_name};
use crate::basics::presets::BobPreset;
use crate::basics::result::BobError;


/// Save the permissions to a file.
#[command]
#[aliases("s")]
#[only_in(guilds)]
pub async fn save(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("Running command: !save");

    let guild = get_guild(&ctx.cache, &msg).await?;
    let channel = get_channel(&ctx.cache, &msg).await?;
    broadcast_typing(&ctx.http, &channel).await?;

    let voice_channel = get_voice_channel(&ctx.http, &guild, &msg).await?.ok_or(
        BobError {msg: "User is not in a voice channel"}
    )?;
    let preset_name = parse_preset_name(&mut args)?;

    let preset = BobPreset::create_from_voice_channel(&voice_channel)?;
    preset.write_guild(&guild.id, &preset_name)?;

    msg.reply(
        &ctx.http,
        format!(
            "📁 Saved permissions of {} to preset `{}`!",
            &voice_channel.mention(),
            &preset_name
        )
    ).await?;

    Ok(())
}

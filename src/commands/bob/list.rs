use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::basics::command::{get_guild, broadcast_typing, get_channel};
use crate::basics::presets::BobPreset;


/// Build a new temporary channel with the specified preset.
#[command]
#[only_in(guilds)]
pub async fn list(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    debug!("Running command: !list");

    let guild = get_guild(&ctx.cache, &msg).await?;
    let channel = get_channel(&ctx.cache, &msg).await?;
    broadcast_typing(&ctx.http, &channel).await?;

    let presets: String = BobPreset::guild_presets_file_list(&guild.id)?
        .into_iter()
        .map(|s| {
            format!("- `{}`", &s.with_extension("").file_name().expect("File had no name").to_string_lossy())
        })
        .collect();

    msg.reply(
        &ctx.http,
        format!(
            "🗒 The following presets are available in **{}**:\n{}", &guild.name, &presets
        )
    ).await?;

    Ok(())
}

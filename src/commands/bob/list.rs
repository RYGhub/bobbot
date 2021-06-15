use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::basics::*;


/// Build a new temporary channel with the specified preset.
#[command]
#[only_in(guilds)]
pub async fn list(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("Running command: !list");

    let guild = get_guild(&msg, &ctx.cache).await;
    let command_channel = get_channel(&msg, &ctx.cache).await;
    let category = get_category(&command_channel, &ctx.http).await;
    broadcast_typing(&command_channel, &ctx.http);

    let presets = get_guild_presets_filenames(&guild.id);
    let presets: String = presets.into_iter().map(|s| {
        format!("- `{}`", &s.file_name().expect("Guild preset file has no name").to_string_lossy())
    }).collect();

    info!("Successfully displayed presets list!");
    msg.channel_id.say(
        &ctx.http,
        format!(
            "🗒 The following presets are available in **{}**:\n{}", &guild.name, &presets
        )
    ).await;

    Ok(())
}

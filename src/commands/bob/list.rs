use std::env;
use std::fs;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::checks::sent_in_bob::*;


/// Build a new temporary channel with the specified preset.
#[command]
#[only_in(guilds)]
#[checks(SentInBob)]
pub async fn list(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    debug!("Running command: !list");

    debug!("Getting guild...");
    let guild = msg.guild(&ctx.cache).await.unwrap();
    debug!("Guild is: {}", &guild.name);

    debug!("Getting command channel...");
    let command_channel = msg.channel(&ctx.cache).await.unwrap().guild().unwrap();
    debug!("Command channel is: #{}", &command_channel.name);

    debug!("Broadcasting typing...");
    command_channel.broadcast_typing(&ctx.http).await?;

    debug!("Getting working directory...");
    let current_path = env::current_dir().expect("Could not get working directory");

    let presets_dir = current_path.join("presets");
    debug!("Accessing presets directory: {}", &presets_dir.to_string_lossy());

    let guild_dir = presets_dir.join(format!("{}", guild.id));
    debug!("Accessing guild presets directory: {}", &guild_dir.to_string_lossy());

    let preset_files = fs::read_dir(guild_dir).expect("Could not read directory contents");
    let preset_files = preset_files.map(|preset_file| {
        let preset_file = preset_file.expect("Could not read directory contents");
        let preset_file = preset_file.path();
        debug!("Found: {}", &preset_file.to_string_lossy());
        let preset_file = preset_file.with_extension("");
        let preset_file = preset_file.file_name().unwrap();
        let preset_file = preset_file.to_string_lossy();
        format!("- `{}`\n", preset_file)
    });
    let preset_files: String = preset_files.collect();

    debug!("Sending channel created message...");
    msg.channel_id.say(
        &ctx.http,
        format!(
            "🗒 The following presets are available in **{}**:\n{}", &guild.name, &preset_files
        )
    ).await?;

    info!("Successfully displayed presets list!");
    Ok(())
}

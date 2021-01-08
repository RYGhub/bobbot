use std::env;
use std::fs;
use std::io::Write;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::checks::sent_in_bob::*;
use crate::checks::bob_has_category::*;
use crate::checks::author_connected_to_voice::*;
use crate::checks::preset_has_valid_name::*;

use crate::utils::{kebabify, BobPreset};


/// Save the permissions to a file.
#[command]
#[aliases("s")]
#[only_in(guilds)]
#[checks(SentInBob, BobHasCategory, AuthorConnectedToVoice, PresetHasValidName)]
pub async fn save(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    debug!("Running command: !save");

    debug!("Getting guild...");
    let guild = msg.guild(&ctx.cache).await.unwrap();
    debug!("Guild is: {}", &guild.name);

    debug!("Getting message author voice state...");
    let author_voice_state = guild.voice_states.get(&msg.author.id).unwrap();

    debug!("Parsing args...");
    let preset_name = kebabify(args.rest());
    debug!("Preset name will be: #{}", &preset_name);

    debug!("Finding message author's voice channel...");
    let author_voice_channel = author_voice_state.channel_id.unwrap();
    let author_voice_channel = author_voice_channel.to_channel(&ctx.http).await?;
    let author_voice_channel = author_voice_channel.guild().unwrap();
    debug!("Message author's voice channel is: #{}", &author_voice_channel.name);

    debug!("Creating preset...");
    let preset = BobPreset {
        permissions: author_voice_channel.permission_overwrites.clone(),
        bitrate: author_voice_channel.bitrate.expect("Voice channel has no bitrate").clone(),
        user_limit: author_voice_channel.user_limit.clone(),
    };

    debug!("Serializing preset into TOML...");
    let serialized = toml::to_string(&preset).expect("Failed to serialize preset");
    debug!("Serializing preset into writable bytes...");
    let serialized = serialized.into_bytes();

    debug!("Getting working directory...");
    let current_path = env::current_dir().expect("Could not get working directory");

    let presets_dir = current_path.join("presets");
    debug!("Accessing/creating presets directory: {}", &presets_dir.to_string_lossy());
    let _ = fs::create_dir(&presets_dir);

    let guild_dir = presets_dir.join(format!("{}", guild.id));
    debug!("Accessing/creating guild presets directory: {}", &guild_dir.to_string_lossy());
    let _ = fs::create_dir(&guild_dir);

    let file_name = format!("{}.toml", &preset_name);
    let preset_path = guild_dir.join(&file_name);
    debug!("Creating/overwriting guild preset file: {}", &preset_path.to_string_lossy());
    let mut preset_file = fs::File::create(&preset_path).expect("Could not create preset file");

    debug!("Writing bytes on the file...");
    let write_result = preset_file.write_all(&serialized);
    if write_result.is_err() {
        error!("Failed to write preset file: {}", &preset_path.to_string_lossy());
        return CommandResult::Err(CommandError::from("Could not write preset file"))
    }

    debug!("Sending preset created message...");
    msg.channel_id.say(&ctx.http, format!("📁 Saved permissions of {} to preset `{}`!", &author_voice_channel.mention(), &preset_name)).await?;

    info!("Successfully created preset {}!", &preset_name);
    Ok(())
}

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

use crate::utils::{kebabify, PermissionOverwritesContainer};


/// Save the permissions to a file.
#[command]
#[only_in(guilds)]
#[checks(SentInBob, BobHasCategory, AuthorConnectedToVoice)]
pub fn save(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    debug!("Running command: !save");

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild = guild.read();
    let author_voice_state = guild.voice_states.get(&msg.author.id).unwrap();

    let preset_name = kebabify(args.rest());

    let author_voice_channel = author_voice_state.channel_id.unwrap();
    let author_voice_channel = author_voice_channel.to_channel(&ctx.http)?;
    let author_voice_channel = author_voice_channel.guild().unwrap();
    let author_voice_channel = author_voice_channel.read();

    let permission_overwrites = &author_voice_channel.permission_overwrites;
    let serialized_overwrites = toml::to_string(&PermissionOverwritesContainer{permissions: permission_overwrites.clone()}).expect("Failed to convert permission overwrites to JSON");
    let serialized_overwrites = serialized_overwrites.into_bytes();

    let current_path = env::current_dir().expect("Could not get current working directory");
    let presets_dir = current_path.join("presets");
    let _ = fs::create_dir(&presets_dir);
    let guild_dir = presets_dir.join(format!("{}", guild.id));
    let _ = fs::create_dir(&guild_dir);
    let preset_path = guild_dir.join(format!("{}.toml", &preset_name));
    let mut preset_file = fs::File::create(&preset_path).expect("Could not create preset file");
    let write_result = preset_file.write_all(&serialized_overwrites);
    if write_result.is_err() {
        return CommandResult::Err(CommandError::from("Could not write preset file"))
    }

    msg.channel_id.say(&ctx.http, format!("📁 Saved permissions of <#{}> to preset `{}`.", &author_voice_channel.id, &preset_name))?;

    Ok(())
}
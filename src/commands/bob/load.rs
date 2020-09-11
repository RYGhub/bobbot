use std::env;
use std::fs;
use std::io::Read;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::checks::sent_in_bob::*;
use crate::checks::bob_has_category::*;
use crate::checks::author_connected_to_voice::*;
use crate::checks::preset_exists::*;

use crate::utils::{kebabify, PermissionOverwritesContainer};
use crate::utils::create_temp_channel::create_temp_channel;


/// Build a new temporary channel with the specified preset.
#[command]
#[only_in(guilds)]
#[checks(SentInBob, BobHasCategory, AuthorConnectedToVoice, PresetExists)]
pub fn load(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("Running command: !load");

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild = guild.read();
    let channel = msg.channel(&ctx.cache).unwrap().guild().unwrap();
    let channel = channel.read();
    let category = channel.category_id.unwrap().to_channel(&ctx.http).unwrap().category().unwrap();
    let category = category.read();

    let preset_name: String = args.single()?;
    let new_channel_name = kebabify(args.rest());

    debug!("Starting to type");
    channel.broadcast_typing(&ctx.http)?;

    debug!("Temp channel permissions will be loaded from the preset {}", &preset_name);
    let current_path = env::current_dir().expect("Could not get current working directory");
    let presets_dir = current_path.join("presets");
    let guild_dir = presets_dir.join(format!("{}", guild.id));
    let preset_path = guild_dir.join(format!("{}.toml", &preset_name));
    let mut preset_file = fs::File::open(preset_path).unwrap();
    let mut serialized_overwrites = Vec::new();
    preset_file.read_to_end(&mut serialized_overwrites).expect(&*format!("Could not read file for preset {}", &preset_name));
    let serialized_overwrites = String::from_utf8(serialized_overwrites).expect(&*format!("Could not create UTF-8 string for preset {}", &preset_name));
    let permission_overwrites: PermissionOverwritesContainer = toml::from_str(&*serialized_overwrites).expect(&*format!("Could not parse file for preset {}", &preset_name));

    debug!("Creating temp channel");
    let created = create_temp_channel(ctx, &guild, &category.id, &new_channel_name, permission_overwrites.permissions.clone())?;

    debug!("Sending channel created message");
    msg.channel_id.say(&ctx.http, format!("🔨 Temp channel <#{}> was built with permissions from the preset `{}`.", &created.id, &preset_name))?;

    debug!("Moving command caller to the created channel");
    guild.move_member(&ctx.http, &msg.author.id, &created.id)?;

    debug!("Build command executed successfully!");

    Ok(())
}

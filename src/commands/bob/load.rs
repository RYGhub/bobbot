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

    let preset_name: String = args.single()?;
    let new_channel_name = kebabify(args.rest());

    channel.broadcast_typing(&ctx.http)?;
    debug!("Started typing");

    let created = guild.create_channel(&ctx.http, |c| {
        debug!("Temp channel name will be: {}", &new_channel_name);
        c.name(new_channel_name);

        debug!("Temp channel type will be: Voice");
        c.kind(ChannelType::Voice);

        debug!("Temp channel category will be: {}", &channel.category_id.unwrap());
        c.category(&channel.category_id.unwrap());

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
        c.permissions(permission_overwrites.permissions);

        c
    })?;
    info!("Created temp channel #{}", &created.name);

    msg.channel_id.say(&ctx.http, format!("🔨 Temp channel <#{}> was built with permissions from the preset `{}`.", &created.id, &preset_name))?;
    debug!("Sent channel created message");

    guild.move_member(&ctx.http, &msg.author.id, &created.id)?;
    debug!("Moved command caller to the created channel");

    Ok(())
}

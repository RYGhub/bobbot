use std::env;
use std::fs;
use std::io::Read;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;

use crate::utils::{kebabify, BobPreset};
use crate::utils::create_temp_channel::create_temp_channel;


/// Build a new temporary channel with the specified preset.
#[command]
#[aliases("l")]
#[only_in(guilds)]
pub async fn load(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("Running command: !load");

    debug!("Getting guild...");
    let guild = msg.guild(&ctx.cache).await.unwrap();
    debug!("Guild is: {}", &guild.name);

    debug!("Getting command channel...");
    let command_channel = msg.channel(&ctx.cache).await.unwrap().guild().unwrap();
    debug!("Command channel is: #{}", &command_channel.name);

    debug!("Getting category...");
    let category_channel = command_channel.category_id.unwrap().to_channel(&ctx.http).await.unwrap().category().unwrap();
    debug!("Category channel is: #{}", &category_channel.name);

    debug!("Parsing args...");
    let preset_name: String = args.single().unwrap();
    debug!("Preset name is: {}", &preset_name);
    let new_channel_name = kebabify(args.rest());
    debug!("Channel name will be: #{}", &new_channel_name);

    debug!("Broadcasting typing...");
    command_channel.broadcast_typing(&ctx.http).await?;

    debug!("Getting default channel permissions from the Bob category...");
    let mut permissions = category_channel.permission_overwrites.clone();

    debug!("Getting working directory...");
    let current_path = env::current_dir().expect("Could not get working directory");

    let presets_dir = current_path.join("presets");
    debug!("Accessing presets directory: {}", &presets_dir.to_string_lossy());

    let guild_dir = presets_dir.join(format!("{}", guild.id));
    debug!("Accessing guild presets directory: {}", &guild_dir.to_string_lossy());

    let file_name = format!("{}.toml", &preset_name);
    let preset_path = guild_dir.join(&file_name);
    debug!("Accessing guild preset file: {}", &preset_path.to_string_lossy());
    let mut preset_file = fs::File::open(&preset_path).expect("Could not open preset file");

    debug!("Reading bytes from the file...");
    let mut serialized = Vec::new();
    preset_file.read_to_end(&mut serialized).expect("Could not read preset file");

    debug!("Deserializing the bytes into the preset...");
    let preset: BobPreset = toml::from_slice(&serialized).expect("Could not deserialize preset");

    debug!("Adding preset permissions...");
    for permission_overwrite in &preset.permissions {
        permissions.push(permission_overwrite.clone())
    }

    debug!("Adding full permissions for channel owner: {}", &msg.author.mention());
    permissions.push(PermissionOverwrite{
        allow: Permissions::all(),
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(msg.author.id.clone())
    });

    debug!("Creating channel...");
    let created = create_temp_channel(&ctx, &guild, &category_channel.id, &new_channel_name, permissions, &preset.bitrate, &preset.user_limit).await?;

    debug!("Sending channel created message...");
    msg.channel_id.say(&ctx.http, format!("🔨 Built channel {} with owner {} from preset `{}`!", &created.mention(), &msg.author.mention(), &preset_name)).await?;

    debug!("Moving command caller to the created channel...");
    guild.move_member(&ctx.http, &msg.author.id, &created.id).await?;

    info!("Successfully built channel #{} for {}#{} with preset {}!", &created.name, &msg.author.name, &msg.author.discriminator, &preset_name);
    Ok(())
}

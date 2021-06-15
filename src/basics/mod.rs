use std::path;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use toml;
use serenity::model::prelude::*;
use serenity::framework::standard::*;
use serenity::client::*;
use serenity::http::*;
use crate::utils::kebabify;
use crate::utils::BobPreset;
use std::convert::TryFrom;


/// Get the guild the message was sent in.
pub async fn get_guild(msg: &Message, cache: &Cache) -> Guild {
    debug!("Getting guild...");
    let guild = msg
        .guild(&cache).await.expect("Message did not have a Guild");

    debug!("Guild is: {}", &guild.name);
    guild
}


/// Get the channel the message was sent in.
pub async fn get_channel(msg: &Message, cache: &Cache) -> GuildChannel {
    debug!("Getting command channel...");
    let command_channel = msg
        .channel(&cache).await.expect("Couldn't retrieve channel")
        .guild().expect("Channel wasn't a GuildChannel");

    debug!("Command channel is: #{}", &command_channel.name);
    command_channel
}


/// Get a reference to the VoiceState of an user.
pub fn get_user_voice_state<'a>(guild: &'a Guild, user_id: &UserId) -> &'a VoiceState {
    debug!("Getting voice state of <@{}>...", &user_id);
    guild.voice_states.get(&user_id).expect("Couldn't get voice state of user")
}


/// Get the voice GuildChannel an user is currently in.
pub async fn get_user_voice_channel(http: &Http, guild: &Guild, user_id: &UserId) -> GuildChannel {
    let vs = get_user_voice_state(&guild, &user_id);

    debug!("Getting voice channel of <@{}>", &user_id);
    vs.channel_id.expect("Couldn't get channel id of user's voice state")
        .to_channel(&http).await.expect("Couldn't get channel information of user's voice state")
        .guild().expect("Voice state channel wasn't a GuildChannel")
}


/// Get the voice GuildChannel the author of the given message is currently in.
pub async fn get_author_voice_channel(http: &Http, guild: &Guild, msg: &Message) -> GuildChannel {
    get_user_voice_channel(&http, &guild, &msg.author.id).await
}


/// Get the category of the passed channel.
pub async fn get_category(channel: &GuildChannel, http: &Http) -> ChannelCategory {
    debug!("Getting category...");
    let category_channel = channel
        .category_id.expect("Channel wasn't in a category")
        .to_channel(&http).await.expect("Couldn't retrieve channel info")
        .category().expect("Channel wasn't a ChannelCategory");

    debug!("Category channel is: #{}", &category_channel.name);
    category_channel
}


/// Parse a single argument as a preset name.
pub fn parse_preset_name(args: &mut Args) -> String {
    let preset_name = args.single().expect("Missing preset name argument");
    debug!("Preset name is: {}", &preset_name);
    preset_name
}


/// Parse the rest of the args as a channel name.
pub fn parse_channel_name(args: Args) -> String {
    debug!("Parsing args...");
    let new_channel_name = kebabify(args.rest());

    debug!("Parsed channel name: #{}", &new_channel_name);
    new_channel_name
}


/// Notify the chat channel that the bot is typing.
pub async fn broadcast_typing(channel: &GuildChannel, http: &Http) {
    debug!("Broadcasting typing...");
    channel.broadcast_typing(&http).await.expect("Couldn't broadcast typing");
}


/// Get the permission overwrites of a category.
pub fn get_category_permows(category: &ChannelCategory) -> Vec<PermissionOverwrite> {
    debug!("Getting default channel permissions from the Bob category...");
    let permissions = category.permission_overwrites.clone();

    debug!("Retrieved: {} permission overwrites", &permissions.len());
    permissions
}


/// Get a full PermissionOverwrite of the passed kind.
pub fn get_full_permow(kind: PermissionOverwriteType) -> PermissionOverwrite {
    debug!("Getting full permissions for kind: {:?}", kind);
    PermissionOverwrite {
        allow: Permissions::all(),
        deny: Permissions::empty(),
        kind: kind,
    }
}


/// Get a full permission overwrite for the member with the given id.
pub fn get_member_permow(userid: UserId) -> PermissionOverwrite {
    debug!("Getting full permissions for member: {}", userid);
    get_full_permow(
        PermissionOverwriteType::Member(
            userid
        )
    )
}


/// Get a full permission overwrite for the bot itself.
pub async fn get_own_permow(cache: &Cache) -> PermissionOverwrite {
    debug!("Getting full permissions for this bot");
    get_member_permow(cache.current_user().await.id.clone())
}


/// Get a full permission overwrite for the author of the given message.
pub fn get_author_permow(msg: &Message) -> PermissionOverwrite {
    debug!("Getting full permissions for author of message: {:?}", msg);
    get_member_permow(msg.author.id.clone())
}


/// Get the permission overwrites of a certain channel.
pub fn get_channel_permows(channel: &GuildChannel) -> Vec<PermissionOverwrite> {
    debug!("Getting permissions of channel: {:?}", &channel);
    channel.permission_overwrites.clone()
}


/// Get the bitrate of a certain voice channel.
pub fn get_channel_bitrate(channel: &GuildChannel) -> u32 {
    debug!("Getting bitrate of channel: {:?}", &channel);
    u32::try_from(channel.bitrate.clone().expect("Channel did not have any bitrate")).expect("Bitrate was larger than a u32")
}


/// Get the user limit of a certain voice channel.
pub fn get_channel_user_limit(channel: &GuildChannel) -> Option<u32> {
    debug!("Getting user_limit of channel: {:?}", &channel);
    let user_limit = channel.user_limit.clone();

    match user_limit {
        Some(user_limit) => Some(u32::try_from(user_limit).expect("User limit was larger than a u32")),
        None => None,
    }
}


/// Create and return a GuildChannel.
pub async fn create_channel(
    http: &Http,
    guild: &Guild,
    category_id: &ChannelId,
    name: &str,
    permissions: Vec<PermissionOverwrite>,
    bitrate: u32,
    user_limit: Option<u32>
) -> GuildChannel
{
    debug!("Creating channel...");

    let created = guild.create_channel(&http, |c| {
        debug!("Channel name will be: {}", &name);
        c.name(name);

        debug!("Channel type will be: Voice");
        c.kind(ChannelType::Voice);

        debug!("Channel category will be: {}", &category_id);
        c.category(category_id.clone());

        debug!("Channel permissions will be: {} permission overwrites", &permissions.len());
        c.permissions(permissions);

        debug!("Channel bitrate will be: {}", &bitrate);
        c.bitrate(bitrate);

        if let Some(limit) = user_limit {
            debug!("Channel user limit will be: {}", &limit);
            c.user_limit(limit);
        }
        else {
            debug!("Channel won't have a user limit")
        }

        c
    }).await.expect("Could not create channel");

    info!("Created channel #{}", &created.name);
    created
}


/// Move an user to a voice channel.
pub async fn move_member(http: &Http, guild: &Guild, user_id: &UserId, channel_id: &ChannelId) {
    debug!("Moving <@{}> to <#{}>...", &user_id, &channel_id);
    guild.move_member(&http, user_id.clone(), channel_id.clone()).await.expect("Couldn't move member");
}


/// Get the current working directory.
pub fn get_cwd() -> path::PathBuf {
    debug!("Getting working directory...");
    env::current_dir().expect("Couldn't get working directory")
}


/// Get the presets directory.
pub fn get_presets_dir() -> path::PathBuf {
    let cwd = get_cwd();

    debug!("Getting presets directory...");
    cwd.join("presets")
}


/// Get the presets directory of the given guild.
pub fn get_guild_presets_dir(guild_id: &GuildId) -> path::PathBuf {
    let pd = get_presets_dir();

    debug!("Getting guild presets directory for <G{}>", &guild_id);
    pd.join(format!("{}", &guild_id))
}


/// Get the list of preset filenames of the given guild.
pub fn get_guild_presets_filenames(guild_id: &GuildId) -> Vec<path::PathBuf> {
    let gpd = get_guild_presets_dir(guild_id);

    debug!("Getting guild presets files for <G{}>", &guild_id);
    let preset_files = fs::read_dir(gpd).expect("Could not read guild presets directory contents");
    let preset_files = preset_files.map(|preset_file| {
        let preset_file = preset_file.expect("Could not read guild preset file");
        let preset_file = preset_file.path();
        debug!("Found: {}", &preset_file.to_string_lossy());
        let preset_file = preset_file.with_extension("");
        preset_file
    });
    preset_files.collect()
}


/// Get the preset filename for the given guild and name.
pub fn get_guild_preset_filename(guild_id: &GuildId, preset_name: &str) -> path::PathBuf {
    let gpd = get_guild_presets_dir(guild_id);

    debug!("Getting filename of guild preset {} of <G{}>", &preset_name, &guild_id);
    gpd.join(format!("{}.toml", &preset_name))
}


/// Read the given preset file into a BobPreset.
pub fn read_preset(file_path: &path::PathBuf) -> BobPreset {
    debug!("Reading preset file {}", &file_path.to_string_lossy());

    let mut file = fs::File::open(&file_path).expect("Could not open preset file");

    let mut serialized = Vec::new();
    file.read_to_end(&mut serialized).expect("Could not read preset file");

    toml::from_slice(&serialized).expect("Could not deserialize preset")
}


/// Read the BobPreset for the given guild and name.
pub fn read_guild_preset(guild_id: &GuildId, preset_name: &str) -> BobPreset {
    let gpf = get_guild_preset_filename(&guild_id, preset_name);

    debug!("Reading guild preset {} of <G{}>", &preset_name, &guild_id);
    read_preset(&gpf)
}


/// Write the given BobPreset into the given file.
pub fn write_preset(file_path: &path::PathBuf, preset: &BobPreset) {
    debug!("Writing preset file {}", &file_path.to_string_lossy());

    let mut file = fs::File::create(&file_path).expect("Could not create preset file");

    let serialized = toml::to_string(&preset).expect("Could not serialize preset").into_bytes();
    file.write_all(&serialized).expect("Could not write preset file");
}


/// Write the given BobPreset for the given guild and name.
pub fn write_guild_preset(guild_id: &GuildId, preset_name: &str, preset: &BobPreset) {
    let gpf = get_guild_preset_filename(&guild_id, preset_name);

    debug!("Writing guild preset {} of <G{}>", &preset_name, &guild_id);
    write_preset(&gpf, &preset);
}


/// Create a BobPreset using the given channel as template.
pub fn get_voice_channel_preset(channel: &GuildChannel) -> BobPreset {
    debug!("Creating preset using {:?} as base...", &channel);
    BobPreset {
        permows: get_channel_permows(&channel),
        bitrate: get_channel_bitrate(&channel),
        user_limit: get_channel_user_limit(&channel),
    }
}


/// Create a BobPreset using the channel the given user is currently in as template.
pub async fn get_user_preset(http: &Http, guild: &Guild, user_id: &UserId) -> BobPreset {
    debug!("Creating preset using {:?} as base...", &user_id);
    get_voice_channel_preset(&get_user_voice_channel(&http, &guild, &user_id).await)
}


/// Create a BobPreset using the channel the message author is currently in as template.
pub async fn get_author_preset(http: &Http, guild: &Guild, msg: &Message) -> BobPreset {
    debug!("Creating preset using {:?} as base...", &msg);
    get_user_preset(&http, &guild, &msg.author.id).await
}
